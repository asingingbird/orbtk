use crate::event::EventBox;

/// This trait is used to define an event handler.
pub trait EventHandler {
    /// Handles an `event` by the given `widget`. If it returns `true` the event will not be forwarded.
    fn handle_event(&self, event: &EventBox) -> bool;

    /// Check if the handler could handle the given event box.
    fn handles_event(&self, event: &EventBox) -> bool;
}
