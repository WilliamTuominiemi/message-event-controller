use event_type::EventType;
mod event_type;

mod event;
use event::Event;

mod event_handler;
use event_handler::EventHandler;

mod log_event_A_handler;
use log_event_A_handler::LogEventAHandler;

mod log_event_B_handler;
use log_event_B_handler::LogEventBHandler;

mod log_event_C_handler;
use log_event_C_handler::LogEventCHandler;

fn main() {
    println!("Hello, world!");
}
