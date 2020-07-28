use crate::*;
use std::fmt;
use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

/// Trait to allow for the [`Object<T>.children`] field to work.
///
/// Should use other traits to define more specifically an object.
pub trait Child: Renderable + Clickable + ChildClone + Downcast {}

impl_downcast!(Child);

pub trait ChildClone {
    fn clone_box(&self) -> Box<dyn Child>;
}

impl<T> ChildClone for T
where
    T: 'static + Child + Clone
{
    fn clone_box(&self) -> Box<dyn Child> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Child> {
    fn clone(&self) -> Box<dyn Child> {
        self.clone_box()
    }
}

/// TODO: Determine how to compare dyn Child.
impl PartialEq for dyn Child + '_ {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// TODO: Determine how to format Child without recursion.
impl fmt::Debug for dyn Child + '_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

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

/// Trait for making object clickable.
///
/// TODO: Should be renamed to something more general like `Eventable` to allow for other javascrtipt methods.
pub trait Clickable {
    /// Method to make a struct listen to the javascript on_click event.
    fn on_click(&mut self, app: &mut App);
}
