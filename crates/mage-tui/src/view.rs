use iced::widget::{column, container, row, scrollable, text, text_input, Column, Row};
use iced::{Color, Element, Font, Length};

use crate::app::{input_id, output_scroll_id, MageShell, Message, Mode, OutputKind};

pub fn view(shell: &MageShell) -> Element<'_, Message> {
    column![
        view_status_bar(shell),
        view_content_area(shell),
        view_input_area(shell),
        view_keybindings_bar(shell),
    ]
    .height(Length::Fill)
    .into()
}

fn view_status_bar(shell: &MageShell) -> Element<'_, Message> {
    let git_info = shell
        .git_branch
        .as_ref()
        .map(|b| format!("  ({})  ", b))
        .unwrap_or_default();

    container(
        row![
            text(format!("  {}  ", shell.cwd)).color(Color::WHITE),
            text(git_info).color(Color::from_rgb(0.0, 0.8, 0.0)),
            text(format!("  {:?}  ", shell.mode)).color(Color::from_rgb(0.9, 0.9, 0.0)),
        ]
        .spacing(4),
    )
    .padding(4)
    .width(Length::Fill)
    .into()
}

fn view_input_area(shell: &MageShell) -> Element<'_, Message> {
    let prompt = if shell.is_executing {
        "Running..."
    } else {
        "> "
    };

    let input = text_input(prompt, &shell.input)
        .id(input_id())
        .on_input(Message::InputChanged)
        .on_submit(Message::InputSubmit)
        .font(Font::MONOSPACE)
        .size(14)
        .padding(8);

    container(input).padding(4).width(Length::Fill).into()
}

fn view_content_area(shell: &MageShell) -> Element<'_, Message> {
    match (shell.panels.output, shell.panels.context_menu) {
        (true, true) => row![
            container(view_output_panel(shell)).width(Length::FillPortion(3)),
            container(view_context_panel(shell)).width(Length::FillPortion(2)),
        ]
        .height(Length::Fill)
        .spacing(4)
        .into(),
        (true, false) => container(view_output_panel(shell))
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        (false, true) => container(view_context_panel(shell))
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        (false, false) => container(
            text("Panels hidden (Ctrl+O output, Ctrl+E context)")
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .center(Length::Fill)
        .into(),
    }
}

fn view_output_panel(shell: &MageShell) -> Element<'_, Message> {
    let lines: Vec<Element<Message>> = shell
        .output
        .iter()
        .map(|line| {
            let color = match line.kind {
                OutputKind::Command => Color::from_rgb(0.0, 0.8, 0.8),
                OutputKind::Error => Color::from_rgb(0.9, 0.2, 0.2),
                OutputKind::Normal => Color::WHITE,
            };
            text(line.text.clone())
                .color(color)
                .font(Font::MONOSPACE)
                .size(13)
                .into()
        })
        .collect();

    container(
        scrollable(Column::with_children(lines).spacing(2).width(Length::Fill))
            .id(output_scroll_id())
            .height(Length::Fill),
    )
    .padding(8)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn view_context_panel(shell: &MageShell) -> Element<'_, Message> {
    let title = if shell.input.is_empty() {
        "Commands"
    } else {
        "Suggestions"
    };

    let items: Vec<Element<Message>> = shell
        .context_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == shell.context_index;
            let color = if is_selected {
                Color::from_rgb(0.0, 0.9, 0.9)
            } else {
                Color::from_rgb(0.7, 0.7, 0.7)
            };

            let prefix = if is_selected { "> " } else { "  " };
            let label = format!(
                "{}[{}] {:<12} {}",
                prefix,
                item.shortcut.unwrap_or(' '),
                item.label,
                item.description
            );

            text(label)
                .font(Font::MONOSPACE)
                .size(13)
                .color(color)
                .into()
        })
        .collect();

    container(
        column![
            text(title).size(14).color(Color::from_rgb(0.8, 0.4, 0.8)),
            Column::with_children(items).spacing(2),
        ]
        .spacing(8),
    )
    .padding(8)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn view_keybindings_bar(shell: &MageShell) -> Element<'_, Message> {
    let keybindings: Vec<(&str, &str)> = match shell.mode {
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
        _ => vec![("Esc", "close")],
    };

    let spans: Vec<Element<Message>> = keybindings
        .iter()
        .flat_map(|(key, action)| {
            vec![
                container(
                    text(format!(" {} ", key))
                        .size(12)
                        .color(Color::from_rgb(0.8, 0.8, 0.8)),
                )
                .into(),
                text(format!("{} ", action))
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
                    .into(),
            ]
        })
        .collect();

    container(Row::with_children(spans).spacing(2))
        .padding(4)
        .width(Length::Fill)
        .into()
}
