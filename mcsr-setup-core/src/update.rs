use crate::{message::Message, state::PageState};

pub fn update(current_page: &mut PageState, message: Message) {
    match message {
        Message::ChangePage(page) => current_page.set_page(page),
        Message::ChangeSetting => (),
    }
}
