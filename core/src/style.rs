use crate::*;
use std::fmt;

/// Type used for events methods.
pub type EventFunction<T> = fn(&mut App, &mut Object<T>) -> ();

/// Base style structure for objects.
///
/// The goal is to have a similar syntax to CSS3.
///
/// There is a potential that this will be move to some more generic definition like `Properties` which will make more sense for event fields like `on_click`.
///
/// Other option is to move event fields to a `Events` property.
#[derive(Clone, Copy)]
pub struct Style<T>
where
    T: Clone,
{
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub on_click: EventFunction<T>,
    pub font: &'static str,
}

impl<T> fmt::Debug for Style<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {}", self.left, self.top, self.width, self.height, self.font)
    }
}

impl<T> Default for Style<T>
where
    T: Clone,
{
    fn default() -> Style<T> {
        Style {
            left: 0f64,
            top: 0f64,
            width: 100f64,
            height: 100f64,
            on_click: |_app: &mut App, _obj: &mut Object<T>| {},
            font: "12px sans-serif",
        }
    }
}

impl<T> PartialEq for Style<T>
where
    T: Clone
{
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && 
        self.top == other.top && 
        self.width == other.width && 
        self.height == other.height
    }
}