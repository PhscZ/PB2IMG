mod state;
mod message;
mod view;

use iced::{event, Subscription};
use message::Message;
use state::AppState;

fn main() -> iced::Result {
    iced::application(
        AppState::default,
        update,
        view::view,
    )
    .title("Iced 0.14 Basic App")
    .subscription(subscription)
    .run()
}

fn update(state: &mut AppState, message: Message) {
    match message {
        Message::Increment => state.counter += 1,
        Message::Decrement => state.counter -= 1,
        Message::Exit => {
            // Tell Iced to quit cleanly
            std::process::exit(0);
        }
    }
}

fn subscription(_state: &AppState) -> Subscription<Message> {
    event::listen().filter_map(|event| match event {
        iced::Event::Window(iced::window::Event::CloseRequested) => {
            Some(Message::Exit)
        }
        _ => None,
    })
}
