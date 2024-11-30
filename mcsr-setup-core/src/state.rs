use iced::{
    font::Weight,
    widget::{column, text},
    Element, Font,
};

use crate::{message::Message, page::Page, settings::config::Config};

#[derive(Default)]
pub struct PageState {
    pub page: Page,
    pub config: Config,
}

impl PageState {
    pub fn set_page(&mut self, page: Page) {
        self.page = page;
    }

    pub fn get_contents(&self) -> Element<Message> {
        let mut font = Font::DEFAULT;
        font.weight = Weight::Bold;

        column([
            text(self.page.get_desc())
                .font(font)
                .size(21)
                .center()
                .into(),
            self.page.get_contents().into(),
        ])
        .into()
    }
}
