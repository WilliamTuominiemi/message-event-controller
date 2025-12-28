use crate::event::Event;
use crate::event_type::EventType;

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(&self, event: Event) -> EventType {
        return event.get_event();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_event() {
        let event = Event::new(EventType::B, "test".to_string());

        let eventHandler = EventHandler {};

        let eventHandlerResponse = eventHandler.handle(event);

        assert_eq!(eventHandlerResponse, EventType::B);
    }
}
