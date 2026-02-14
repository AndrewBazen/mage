mod app;
mod config;
mod interpreter;
mod view;

use app::MageShell;
use iced::Font;

fn main() -> iced::Result {
    iced::application(MageShell::new, MageShell::update, MageShell::view)
        .title("Mage Shell")
        .subscription(MageShell::subscription)
        .theme(MageShell::theme)
        .default_font(Font::MONOSPACE)
        .window_size((1200.0, 800.0))
        .run()
}
