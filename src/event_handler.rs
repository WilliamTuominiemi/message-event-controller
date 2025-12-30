use crate::event::Event;
use crate::event_type::EventType;

pub trait EventHandler {
    fn event_type(&self) -> EventType;
    fn handle(&self, event: &Event);
}
