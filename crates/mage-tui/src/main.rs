mod app;
mod ui;
mod widgets;
mod config;

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use app::App;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Global keybindings
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') | KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.toggle_command_palette(),
                    KeyCode::Char('f') => app.toggle_file_browser(),
                    KeyCode::Char('g') => app.toggle_git_panel(),
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc => app.handle_escape(),
                    KeyCode::Enter => app.handle_enter(),
                    KeyCode::Tab => app.handle_tab(),
                    KeyCode::Backspace => app.handle_backspace(),
                    KeyCode::Char(c) => app.handle_char(c),
                    KeyCode::Up => app.handle_up(),
                    KeyCode::Down => app.handle_down(),
                    KeyCode::Left => app.handle_left(),
                    KeyCode::Right => app.handle_right(),
                    _ => {}
                }
            }
        }
    }
}
