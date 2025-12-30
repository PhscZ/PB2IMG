use iced::Command;

use crate::message::Message;
use crate::state::AppState;

pub fn boot() -> AppState {
    AppState::default()
}

pub fn update(
    state: &mut AppState,
    message: Message,
) -> Command<Message> {
    match message {
        Message::Increment => state.counter += 1,
        Message::Decrement => state.counter -= 1,
    }

    Command::none()
}
