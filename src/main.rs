use iced::window::Settings;
use mcsr_setup_core::{update::update, view::view};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.platform_specific.application_id = "MCSR Setup v0.1.0".to_string();
    iced::application("MCSR Setup v0.1.0", update, view)
        .theme(|_| iced::Theme::Nord)
        .window(settings)
        .run()
}
