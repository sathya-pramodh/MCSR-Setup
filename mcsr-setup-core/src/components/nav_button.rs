use iced::{
    daemon::DefaultStyle,
    widget::{button, text, Button},
    Length, Theme,
};

use crate::{message::Message, page::Page};

pub fn nav_button<'a>(page: Page) -> Button<'a, Message> {
    let text_color = Theme::Nord.default_style().text_color;
    button(text(page.get_desc()).color(text_color))
        .on_press(Message::ChangePage(page))
        .width(Length::Fill)
        .into()
}
