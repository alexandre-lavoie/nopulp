use crate::app::App;
use crate::core::{Defaultable, Object};

/// Base style structure for objects.
///
/// The goal is to have a similar syntax to CSS3.
///
/// There is a potential that this will be move to some more generic definition like `Properties` which will make more sense for event fields like `on_click`.
/// 
/// Other option is to move event fields to a `Events` property.
#[derive(Clone)]
pub struct Style<T>
where
    T: Clone,
{
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub on_click: fn(&mut App, &mut Object<T>) -> (),
    pub font: &'static str,
}

impl<T> Defaultable<Style<T>> for Style<T>
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