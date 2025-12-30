use crate::event::Event;
use crate::event_handler::EventHandler;
use crate::event_type::EventType;

pub struct LogEventCHandler;

impl EventHandler for LogEventCHandler {
    fn event_type(&self) -> EventType {
        EventType::C
    }

    fn handle(&self, event: &Event) {
        println!(
            "[LOG] Event C received at {:?} with payload: {}",
            event.timestamp(),
            event.payload()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_event_A() {
        let event = Event::new(EventType::C, "test".to_string());
        let handler = LogEventCHandler;

        handler.handle(&event);
    }
}
