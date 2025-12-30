use crate::event::Event;
use crate::event_handler::EventHandler;
use crate::event_type::EventType;

pub struct LogEventAHandler;

impl EventHandler for LogEventAHandler {
    fn event_type(&self) -> EventType {
        EventType::A
    }

    fn handle(&self, event: &Event) {
        println!(
            "[LOG] Event A received at {:?} with payload: {}",
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
        let event = Event::new(EventType::A, "test".to_string());
        let handler = LogEventAHandler;

        handler.handle(&event);
    }
}
