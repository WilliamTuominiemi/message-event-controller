use crate::event::Event;
use crate::event_handler::EventHandler;
use crate::event_type::EventType;

pub struct LogEventBHandler;

impl EventHandler for LogEventBHandler {
    fn event_type(&self) -> EventType {
        EventType::B
    }

    fn handle(&self, event: &Event) {
        println!(
            "[LOG] Event B received at {:?} with payload: {}",
            event.timestamp(),
            event.payload()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_event_B() {
        let event = Event::new(EventType::B, "test".to_string());
        let handler = LogEventBHandler;

        handler.handle(&event);
    }
}
