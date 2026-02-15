use std::sync::mpsc;

use iced::event;
use iced::keyboard;
use iced::widget::Id;
use iced::widget::operation::{self, AbsoluteOffset};
use iced::{Element, Event, Subscription, Task, Theme};

use crate::config::TuiConfig;
use crate::interpreter::CommandResult;
use crate::view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    #[allow(dead_code)]
    Insert,
    #[allow(dead_code)]
    Command,
    CommandPalette,
    FileBrowser,
    GitPanel,
}

#[derive(Debug, Clone)]
pub struct PanelState {
    pub output: bool,
    pub context_menu: bool,
    pub file_browser: bool,
    pub git_panel: bool,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            output: true,
            context_menu: true,
            file_browser: false,
            git_panel: false,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoryEntry {
    pub command: String,
    pub output: String,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub enum OutputKind {
    Command,
    Normal,
    Error,
}

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub kind: OutputKind,
}

#[derive(Debug, Clone)]
pub struct ContextItem {
    pub shortcut: Option<char>,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    InputSubmit,
    ToggleOutputPanel,
    ToggleContextPanel,
    ToggleFileBrowser,
    ToggleGitPanel,
    ToggleCommandPalette,
    HistoryUp,
    HistoryDown,
    ScrollUp,
    ScrollDown,
    TabComplete,
    EscapePressed,
    CommandComplete(CommandResult),
    #[allow(dead_code)]
    ContextItemSelected(usize),
    QuitRequested,
    InterpreterReady(mpsc::Sender<String>),
}

pub struct MageShell {
    pub input: String,
    pub mode: Mode,
    pub panels: PanelState,
    pub history: Vec<HistoryEntry>,
    pub history_index: Option<usize>,
    pub output: Vec<OutputLine>,
    pub cwd: String,
    pub git_branch: Option<String>,
    pub context_items: Vec<ContextItem>,
    pub context_index: usize,
    pub is_executing: bool,
    pub cmd_tx: Option<mpsc::Sender<String>>,
    #[allow(dead_code)]
    pub config: TuiConfig,
}

impl MageShell {
    pub fn new() -> (Self, Task<Message>) {
        let config = TuiConfig::load();

        let cwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "~".to_string());

        let git_branch = detect_git_branch();

        let mut shell = Self {
            input: String::new(),
            mode: Mode::Normal,
            panels: PanelState::default(),
            history: Vec::new(),
            history_index: None,
            output: vec![OutputLine {
                text: "Welcome to Mage Shell!".to_string(),
                kind: OutputKind::Normal,
            }],
            cwd,
            git_branch,
            context_items: Vec::new(),
            context_index: 0,
            is_executing: false,
            cmd_tx: None,
            config: config.clone(),
        };

        // Apply config panel defaults
        shell.panels.output = config.layout.output_width > 0;
        shell.panels.context_menu = config.layout.context_width > 0;
        shell.update_context();

        (shell, operation::focus(input_id()))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InterpreterReady(tx) => {
                self.cmd_tx = Some(tx);
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input = value;
                self.update_context();
                Task::none()
            }
            Message::InputSubmit => {
                if self.input.is_empty() || self.is_executing {
                    return Task::none();
                }

                let command = self.input.clone();
                self.input.clear();

                // Echo the command
                self.output.push(OutputLine {
                    text: format!("> {}", command),
                    kind: OutputKind::Command,
                });

                // Send to interpreter thread
                if let Some(tx) = &self.cmd_tx {
                    let _ = tx.send(command);
                    self.is_executing = true;
                }

                self.update_context();
                operation::snap_to_end(output_scroll_id())
            }
            Message::CommandComplete(result) => {
                let mut output_text = String::new();

                for line in &result.stdout_lines {
                    self.output.push(OutputLine {
                        text: line.clone(),
                        kind: OutputKind::Normal,
                    });
                    if !output_text.is_empty() {
                        output_text.push('\n');
                    }
                    output_text.push_str(line);
                }

                for line in &result.stderr_lines {
                    self.output.push(OutputLine {
                        text: format!("[err] {}", line),
                        kind: OutputKind::Error,
                    });
                    if !output_text.is_empty() {
                        output_text.push('\n');
                    }
                    output_text.push_str(line);
                }

                if result.stdout_lines.is_empty()
                    && result.stderr_lines.is_empty()
                    && result.success
                {
                    output_text = "OK".to_string();
                }

                self.history.push(HistoryEntry {
                    command: result.command,
                    output: output_text,
                    success: result.success,
                });
                self.history_index = None;
                self.is_executing = false;

                operation::snap_to_end(output_scroll_id())
            }
            Message::HistoryUp => {
                if self.history.is_empty() {
                    return Task::none();
                }

                let new_index = match self.history_index {
                    None => self.history.len() - 1,
                    Some(0) => 0,
                    Some(i) => i - 1,
                };

                self.history_index = Some(new_index);
                self.input = self.history[new_index].command.clone();
                self.update_context();
                operation::move_cursor_to_end(input_id())
            }
            Message::HistoryDown => {
                if let Some(index) = self.history_index {
                    if index + 1 < self.history.len() {
                        self.history_index = Some(index + 1);
                        self.input = self.history[index + 1].command.clone();
                    } else {
                        self.history_index = None;
                        self.input.clear();
                    }
                    self.update_context();
                    operation::move_cursor_to_end(input_id())
                } else {
                    Task::none()
                }
            }
            Message::TabComplete => {
                if !self.context_items.is_empty() {
                    let item = &self.context_items[self.context_index];
                    self.input = format!("{} ", item.label);
                    self.context_index = (self.context_index + 1) % self.context_items.len();
                    self.update_context();
                    operation::move_cursor_to_end(input_id())
                } else {
                    Task::none()
                }
            }
            Message::EscapePressed => {
                match self.mode {
                    Mode::Insert | Mode::Command => self.mode = Mode::Normal,
                    Mode::CommandPalette => self.mode = Mode::Normal,
                    Mode::FileBrowser => {
                        self.panels.file_browser = false;
                        self.mode = Mode::Normal;
                    }
                    Mode::GitPanel => {
                        self.panels.git_panel = false;
                        self.mode = Mode::Normal;
                    }
                    Mode::Normal => {}
                }
                Task::none()
            }
            Message::ToggleOutputPanel => {
                self.panels.output = !self.panels.output;
                Task::none()
            }
            Message::ToggleContextPanel => {
                self.panels.context_menu = !self.panels.context_menu;
                Task::none()
            }
            Message::ToggleFileBrowser => {
                self.panels.file_browser = !self.panels.file_browser;
                if self.panels.file_browser {
                    self.mode = Mode::FileBrowser;
                } else if self.mode == Mode::FileBrowser {
                    self.mode = Mode::Normal;
                }
                Task::none()
            }
            Message::ToggleGitPanel => {
                self.panels.git_panel = !self.panels.git_panel;
                if self.panels.git_panel {
                    self.mode = Mode::GitPanel;
                } else if self.mode == Mode::GitPanel {
                    self.mode = Mode::Normal;
                }
                Task::none()
            }
            Message::ToggleCommandPalette => {
                self.mode = if self.mode == Mode::CommandPalette {
                    Mode::Normal
                } else {
                    Mode::CommandPalette
                };
                Task::none()
            }
            Message::ScrollUp => {
                operation::scroll_by(output_scroll_id(), AbsoluteOffset { x: 0.0, y: -60.0 })
            }
            Message::ScrollDown => {
                operation::scroll_by(output_scroll_id(), AbsoluteOffset { x: 0.0, y: 60.0 })
            }
            Message::ContextItemSelected(i) => {
                if i < self.context_items.len() {
                    self.input = format!("{} ", self.context_items[i].label);
                    self.update_context();
                    operation::move_cursor_to_end(input_id())
                } else {
                    Task::none()
                }
            }
            Message::QuitRequested => iced::exit(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        view::view(self)
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([keyboard_subscription(), interpreter_subscription()])
    }

    fn update_context(&mut self) {
        self.context_items.clear();
        self.context_index = 0;

        let input_lower = self.input.to_lowercase();

        let keywords = [
            ("conjure", "Declare a variable"),
            ("incant", "Print output"),
            ("evoke", "Execute shell command"),
            ("scry", "Conditional (if)"),
            ("morph", "Else-if branch"),
            ("lest", "Else branch"),
            ("enchant", "Define a function"),
            ("cast", "Call a function"),
            ("chant", "For loop"),
            ("channel", "While loop"),
            ("loop", "Infinite loop"),
            ("invoke", "Try block"),
            ("seal", "Catch block"),
            ("summon", "Throw error"),
            ("bestow", "Return value"),
            ("dispel", "Break loop"),
            ("portal", "Continue loop"),
        ];

        for (keyword, desc) in keywords {
            if keyword.starts_with(&input_lower) || input_lower.is_empty() {
                self.context_items.push(ContextItem {
                    shortcut: keyword.chars().next(),
                    label: keyword.to_string(),
                    description: desc.to_string(),
                });
            }
        }
    }
}

pub fn output_scroll_id() -> Id {
    Id::new("output_scroll")
}

pub fn input_id() -> Id {
    Id::new("command_input")
}

fn detect_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
}

fn handle_event(
    event: Event,
    _status: event::Status,
    _window: iced::window::Id,
) -> Option<Message> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            handle_key_press(key, modifiers)
        }
        _ => None,
    }
}

fn handle_key_press(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    use keyboard::key::Named;

    if modifiers.control() {
        match key.as_ref() {
            keyboard::Key::Character("c") | keyboard::Key::Character("q") => {
                Some(Message::QuitRequested)
            }
            keyboard::Key::Character("p") => Some(Message::ToggleCommandPalette),
            keyboard::Key::Character("f") => Some(Message::ToggleFileBrowser),
            keyboard::Key::Character("g") => Some(Message::ToggleGitPanel),
            keyboard::Key::Character("o") => Some(Message::ToggleOutputPanel),
            keyboard::Key::Character("e") => Some(Message::ToggleContextPanel),
            _ => None,
        }
    } else {
        match key.as_ref() {
            keyboard::Key::Named(Named::Escape) => Some(Message::EscapePressed),
            keyboard::Key::Named(Named::ArrowUp) => Some(Message::HistoryUp),
            keyboard::Key::Named(Named::ArrowDown) => Some(Message::HistoryDown),
            keyboard::Key::Named(Named::PageUp) => Some(Message::ScrollUp),
            keyboard::Key::Named(Named::PageDown) => Some(Message::ScrollDown),
            keyboard::Key::Named(Named::Tab) => Some(Message::TabComplete),
            _ => None,
        }
    }
}

fn keyboard_subscription() -> Subscription<Message> {
    event::listen_with(handle_event)
}

fn interpreter_stream() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(
        100,
        async move |mut output: iced::futures::channel::mpsc::Sender<Message>| {
            let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<String>();
            let (result_tx, result_rx) = std::sync::mpsc::channel::<CommandResult>();

            // Send the command sender back to the app
            use iced::futures::SinkExt;
            let _ = output.send(Message::InterpreterReady(cmd_tx)).await;

            // Spawn the interpreter thread (owns scope + functions, non-Send types stay here)
            std::thread::spawn(move || {
                crate::interpreter::interpreter_thread(cmd_rx, result_tx);
            });

            // Poll for results from the interpreter thread
            loop {
                match result_rx.try_recv() {
                    Ok(result) => {
                        let _ = output.send(Message::CommandComplete(result)).await;
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                }
            }
        },
    )
}

fn interpreter_subscription() -> Subscription<Message> {
    Subscription::run(interpreter_stream)
}
