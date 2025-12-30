use iced::{
    widget::{button, column, row, text},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::state::AppState;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let title = text("PB2IMG").size(28);

    let counter = text(state.counter.to_string()).size(40);

    let controls = row![
        button("-").on_press(Message::Decrement),
        button("+").on_press(Message::Increment),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    column![
        title,
        counter,
        controls
    ]
    .spacing(20)
    .padding(20)
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
