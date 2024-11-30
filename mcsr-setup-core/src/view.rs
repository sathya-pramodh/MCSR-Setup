use iced::{
    widget::{column, row, Rule},
    Element,
};

use crate::{components::nav_button::nav_button, message::Message, page::Page, state::PageState};

pub fn view(page_state: &PageState) -> Element<Message> {
    row([
        column([
            nav_button(Page::HomePage).into(),
            nav_button(Page::SettingsPage(page_state.config)).into(),
        ])
        .padding(10)
        .spacing(10)
        .width(175)
        .clip(true)
        .into(),
        Rule::vertical(10).into(),
        page_state.get_contents().into(),
    ])
    .padding(10)
    .spacing(10)
    .into()
}
