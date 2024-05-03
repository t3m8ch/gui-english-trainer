use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container},
    Alignment, Element, Length,
};

use crate::messages::Message;

pub fn app_container<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(content)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
}

pub fn app_column<'a>() -> Column<'a, Message> {
    Column::new().align_items(Alignment::Center).spacing(10)
}
