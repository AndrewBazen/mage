use std::collections::HashMap;
use mage_core::interpreter::{ExprValue, FunctionDef, interpret};
use mage_core::output::OutputCollector;
use mage_core::parser::MageParser;
use mage_core::Rule;
use pest::Parser;

/// The current mode of the application
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

/// Panel visibility state
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

/// Command history entry
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoryEntry {
    pub command: String,
    pub output: String,
    pub success: bool,
}

/// Main application state
pub struct App {
    /// Current input buffer
    pub input: String,
    /// Cursor position in input
    pub cursor_pos: usize,
    /// Current mode
    pub mode: Mode,
    /// Panel visibility
    pub panels: PanelState,
    /// Command history
    pub history: Vec<HistoryEntry>,
    /// History navigation index
    pub history_index: Option<usize>,
    /// Output buffer (recent output)
    pub output: Vec<String>,
    /// Current working directory
    pub cwd: String,
    /// Git branch (if in a git repo)
    pub git_branch: Option<String>,
    /// Context menu items for current input
    pub context_items: Vec<ContextItem>,
    /// Selected context item index
    pub context_index: usize,
    /// Mage interpreter scope (persistent across commands)
    pub scope: HashMap<String, ExprValue>,
    /// Mage function definitions (persistent across commands)
    pub functions: HashMap<String, FunctionDef<'static>>,
    /// Output scroll offset (0 = bottom)
    pub scroll_offset: usize,
}

/// A context menu item (command suggestion)
#[derive(Debug, Clone)]
pub struct ContextItem {
    pub shortcut: Option<char>,
    pub label: String,
    pub description: String,
}

impl App {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "~".to_string());

        let git_branch = Self::detect_git_branch();

        Self {
            input: String::new(),
            cursor_pos: 0,
            mode: Mode::Normal,
            panels: PanelState::default(),
            history: Vec::new(),
            history_index: None,
            output: vec!["Welcome to Mage Shell!".to_string()],
            cwd,
            git_branch,
            context_items: Vec::new(),
            context_index: 0,
            scope: HashMap::new(),
            functions: HashMap::new(),
            scroll_offset: 0,
        }
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

    pub fn toggle_command_palette(&mut self) {
        self.mode = if self.mode == Mode::CommandPalette {
            Mode::Normal
        } else {
            Mode::CommandPalette
        };
    }

    pub fn toggle_file_browser(&mut self) {
        self.panels.file_browser = !self.panels.file_browser;
        if self.panels.file_browser {
            self.mode = Mode::FileBrowser;
        } else if self.mode == Mode::FileBrowser {
            self.mode = Mode::Normal;
        }
    }

    pub fn toggle_git_panel(&mut self) {
        self.panels.git_panel = !self.panels.git_panel;
        if self.panels.git_panel {
            self.mode = Mode::GitPanel;
        } else if self.mode == Mode::GitPanel {
            self.mode = Mode::Normal;
        }
    }

    pub fn toggle_output_panel(&mut self) {
        self.panels.output = !self.panels.output;
    }

    pub fn toggle_context_panel(&mut self) {
        self.panels.context_menu = !self.panels.context_menu;
    }

    pub fn handle_escape(&mut self) {
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
    }

    pub fn handle_enter(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let command = self.input.clone();
        self.input.clear();
        self.cursor_pos = 0;

        // Echo the command
        self.output.push(format!("> {}", command));

        // Leak input string to get 'static lifetime (same pattern as CLI REPL)
        let input: &'static str = Box::leak(command.clone().into_boxed_str());

        // Parse and interpret with persistent scope/functions and buffered output
        let mut collector = OutputCollector::buffered();
        let success = match MageParser::parse(Rule::program, input) {
            Ok(pairs) => {
                match interpret(pairs, None, &mut self.scope, &mut self.functions, &mut collector) {
                    Ok(()) => true,
                    Err(e) => {
                        collector.eprintln(&format!("{}", e));
                        false
                    }
                }
            }
            Err(e) => {
                collector.eprintln(&format!("Parse error: {}", e));
                false
            }
        };

        // Collect captured output
        let stdout_lines = collector.take_stdout();
        let stderr_lines = collector.take_stderr();

        let mut output_text = String::new();

        for line in &stdout_lines {
            self.output.push(line.clone());
            if !output_text.is_empty() {
                output_text.push('\n');
            }
            output_text.push_str(line);
        }
        for line in &stderr_lines {
            self.output.push(format!("[err] {}", line));
            if !output_text.is_empty() {
                output_text.push('\n');
            }
            output_text.push_str(line);
        }

        if stdout_lines.is_empty() && stderr_lines.is_empty() && success {
            output_text = "OK".to_string();
        }

        self.history.push(HistoryEntry {
            command,
            output: output_text,
            success,
        });
        self.history_index = None;

        // Auto-scroll to bottom on new output
        self.scroll_offset = 0;

        self.update_context();
    }

    pub fn handle_tab(&mut self) {
        if !self.context_items.is_empty() {
            self.context_index = (self.context_index + 1) % self.context_items.len();
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.input.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
            self.update_context();
        }
    }

    pub fn handle_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
        self.update_context();
    }

    pub fn handle_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => self.history.len() - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };

        self.history_index = Some(new_index);
        self.input = self.history[new_index].command.clone();
        self.cursor_pos = self.input.len();
    }

    pub fn handle_down(&mut self) {
        if let Some(index) = self.history_index {
            if index + 1 < self.history.len() {
                self.history_index = Some(index + 1);
                self.input = self.history[index + 1].command.clone();
            } else {
                self.history_index = None;
                self.input.clear();
            }
            self.cursor_pos = self.input.len();
        }
    }

    pub fn handle_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn handle_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset + 1 < self.output.len() {
            self.scroll_offset += 3;
            if self.scroll_offset >= self.output.len() {
                self.scroll_offset = self.output.len().saturating_sub(1);
            }
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(3);
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
