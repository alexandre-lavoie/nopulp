use crate::app::App;

/// Trait to allow for the [`Object<T>.children`] field to work.
///
/// Should use other traits to define more specifically an object.
pub trait Child: Renderable + Clickable {}

pub trait Renderable {
    /// Method to make a struct renderable.
    ///
    /// This method should use `App.get_context()` to render.
    fn render(&self, app: &App);
}

pub trait Initializable<T, K> {
    /// Method to initialize a struct.
    ///
    /// TODO: Should use param spread instead.
    fn new(param: T) -> K;
}

pub trait Defaultable<T> {
    /// Method to get default value of a struct.
    fn default() -> T;
}

/// Trait for making object clickable.
///
/// TODO: Should be renamed to something more general like `Eventable` to allow for other javascrtipt methods.
pub trait Clickable {
    /// Method to make a struct listen to the javascript on_click event.
    fn on_click(&mut self, app: &mut App);
}
