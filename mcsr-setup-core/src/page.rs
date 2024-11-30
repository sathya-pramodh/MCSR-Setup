use crate::{components::nav_button::nav_button, message::Message, settings::config::Config};
use iced::{
    widget::{column, container, row, text},
    Element, Length,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum Page {
    #[default]
    HomePage,
    SettingsPage(Config),
    NvidiaPage,
}

impl Page {
    pub fn get_desc(&self) -> String {
        String::from(match self {
            Page::HomePage => "Home",
            Page::SettingsPage(_) => "Settings",
            Page::NvidiaPage => "Nvidia Drivers",
        })
    }

    pub fn get_contents(&self) -> Element<Message> {
        match self {
            Page::SettingsPage(config) => {
                let descs = config.get_descriptors();
                let mut rows: Vec<Element<Message>> = vec![];
                for (field_desc, value) in descs {
                    rows.push(
                        row([text(field_desc.get_desc()).into(), text(value).into()])
                            .spacing(50)
                            .into(),
                    );
                }
                column(rows).width(Length::Fill).into()
            }
            Page::HomePage => container(nav_button(Page::NvidiaPage))
                .max_width(500)
                .into(),
            _ => text("").into(),
        }
    }
}
