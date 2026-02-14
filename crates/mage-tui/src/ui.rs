use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Status bar
            Constraint::Min(10),   // Content area
            Constraint::Length(3), // Input area
            Constraint::Length(1), // Keybindings bar
        ])
        .split(f.area());

    draw_status_bar(f, app, chunks[0]);
    draw_content(f, app, chunks[1]);
    draw_input(f, app, chunks[2]);
    draw_keybindings(f, app, chunks[3]);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let git_info = app.git_branch.as_ref()
        .map(|b| format!(" ({}) ", b))
        .unwrap_or_default();

    let status = Line::from(vec![
        Span::styled(" ", Style::default().bg(Color::Magenta)),
        Span::styled(
            format!(" {} ", app.cwd),
            Style::default().fg(Color::White).bg(Color::DarkGray),
        ),
        Span::styled(
            git_info,
            Style::default().fg(Color::Green).bg(Color::DarkGray),
        ),
        Span::styled(
            format!(" {:?} ", app.mode),
            Style::default().fg(Color::Yellow).bg(Color::DarkGray),
        ),
    ]);

    let status_bar = Paragraph::new(status)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(status_bar, area);
}

fn draw_content(f: &mut Frame, app: &App, area: Rect) {
    let show_output = app.panels.output;
    let show_context = app.panels.context_menu;

    match (show_output, show_context) {
        (true, true) => {
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ])
                .split(area);
            draw_output(f, app, content_chunks[0]);
            draw_context_panel(f, app, content_chunks[1]);
        }
        (true, false) => {
            draw_output(f, app, area);
        }
        (false, true) => {
            draw_context_panel(f, app, area);
        }
        (false, false) => {
            let block = Block::default()
                .title(" Panels hidden (^O output, ^E context) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            f.render_widget(block, area);
        }
    }
}

fn draw_output(f: &mut Frame, app: &App, area: Rect) {
    // Calculate visible area (inside borders)
    let inner_height = area.height.saturating_sub(2) as usize;

    let total = app.output.len();
    let end = total.saturating_sub(app.scroll_offset);
    let start = end.saturating_sub(inner_height);

    let output_items: Vec<ListItem> = app.output[start..end]
        .iter()
        .map(|line| {
            let style = if line.starts_with("> ") {
                Style::default().fg(Color::Cyan)
            } else if line.starts_with("[err]") {
                Style::default().fg(Color::Red)
            } else if line.contains("error") || line.contains("Error") || line.contains("CURSE") {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(line.clone(), style)))
        })
        .collect();

    let scroll_indicator = if app.scroll_offset > 0 {
        format!(" Output [{}/{}] ", end, total)
    } else {
        " Output ".to_string()
    };

    let output = List::new(output_items)
        .block(Block::default()
            .title(scroll_indicator)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)));

    f.render_widget(output, area);
}

fn draw_context_panel(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.context_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let shortcut = item.shortcut
                .map(|c| format!("[{}] ", c))
                .unwrap_or_default();

            let style = if i == app.context_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let content = format!("{}{:<12} {}", shortcut, item.label, item.description);
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let title = if app.input.is_empty() {
        " Commands "
    } else {
        " Suggestions "
    };

    let context = List::new(items)
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta)));

    f.render_widget(context, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .title(" > ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));

    f.render_widget(input, area);

    f.set_cursor_position((
        area.x + app.cursor_pos as u16 + 1,
        area.y + 1,
    ));
}

fn draw_keybindings(f: &mut Frame, app: &App, area: Rect) {
    let keybindings = match app.mode {
        Mode::Normal => vec![
            ("^P", "palette"),
            ("^F", "files"),
            ("^G", "git"),
            ("^O", "output"),
            ("^E", "context"),
            ("^C", "quit"),
            ("PgUp/Dn", "scroll"),
            ("Enter", "run"),
        ],
        Mode::Insert => vec![
            ("Esc", "normal"),
            ("Enter", "run"),
            ("Tab", "complete"),
        ],
        Mode::CommandPalette => vec![
            ("Esc", "close"),
            ("Enter", "select"),
            ("Tab", "next"),
        ],
        _ => vec![
            ("Esc", "close"),
        ],
    };

    let spans: Vec<Span> = keybindings
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default().fg(Color::Black).bg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{} ", action),
                    Style::default().fg(Color::DarkGray),
                ),
            ]
        })
        .collect();

    let bar = Paragraph::new(Line::from(spans));
    f.render_widget(bar, area);
}
