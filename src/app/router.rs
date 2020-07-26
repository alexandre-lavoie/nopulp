use crate::app::App;
use crate::components::html::Html;

/// Trait that defines the routing properties of external application.
pub trait Routeable {
    /// Primary method for routing.
    ///
    /// Should be using the [`router!`] macro.
    fn route(app: &mut App) -> Box<Html>;
}
