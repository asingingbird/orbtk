use std::rc::Rc;

use crate::{prelude::*, shell::KeyEvent};

use super::{Event, EventBox, EventHandler};

pub struct KeyDownEvent {
    pub event: KeyEvent,
}

impl Event for KeyDownEvent {}

pub struct KeyUpEvent {
    pub event: KeyEvent,
}

impl Event for KeyUpEvent {}

pub type KeyHandler = dyn Fn(KeyEvent) -> bool + 'static;

/// Used to handle key down events. Could be attached to a widget.
pub struct KeyDownEventHandler {
    handler: Rc<KeyHandler>,
}

impl Into<Rc<dyn EventHandler>> for KeyDownEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for KeyDownEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        event
            .downcast_ref::<KeyDownEvent>()
            .ok()
            .map_or(false, |event| (self.handler)(event.event.clone()))
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<KeyDownEvent>()
    }
}

pub trait KeyDownHandler: Sized + Widget {
    /// Inserts a handler.
    fn on_key_down<H: Fn(KeyEvent) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(KeyDownEventHandler {
            handler: Rc::new(handler),
        })
    }
}
