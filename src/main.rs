use event_type::EventType;
mod event_type;

mod event;
use event::Event;

mod event_handler;
use event_handler::EventHandler;

fn main() {
    println!("Hello, world!");
}
