use event_type::EventType;
mod event_type;

mod event;
use event::Event;

mod event_handler;
use event_handler::EventHandler;

mod log_event_A_handler;
use log_event_A_handler::LogEventAHandler;

fn main() {
    println!("Hello, world!");
}
