use crate::events::event::Event;

pub enum MouseEventType {
    CLICK,
    MOVE
};

pub struct MouseEvent {
    type: MouseEventType,
    x: f64,
    y: f64
}

impl Event for MouseEvent {
    
}