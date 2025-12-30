use crate::event_type::EventType;

use std::time::SystemTime;

pub struct Event {
    event_type: EventType,
    timestamp: SystemTime,
    payload: String,
}

impl Event {
    pub fn new(event_type: EventType, payload: String) -> Self {
        let timestamp = SystemTime::now();

        Event {
            event_type,
            timestamp,
            payload,
        }
    }

    pub fn event_type(&self) -> EventType {
        return self.event_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let event = Event::new(EventType::A, "test".to_string());

        assert_eq!(event.event_type, EventType::A);
        assert_eq!(event.payload, "test");
    }
}
