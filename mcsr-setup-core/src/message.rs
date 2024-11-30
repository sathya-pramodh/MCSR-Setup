use crate::page::Page;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),
    ChangeSetting,
}
