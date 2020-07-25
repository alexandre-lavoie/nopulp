use crate::core::{Object, Defaultable};
use crate::app::App;

#[derive(Clone)]
pub struct Style<T> where T: Clone {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub on_click: fn(&mut App, &mut Object<T>) -> (),
    pub font: &'static str,
}

impl<T> Defaultable<Style<T>> for Style<T> where T: Clone {
    fn default() -> Style<T> {
        Style {
            left: 0f64,
            top: 0f64,
            width: 100f64,
            height: 100f64,
            on_click: |_app: &mut App, _obj: &mut Object<T>| {},
            font: "12px sans-serif"
        }
    }
}