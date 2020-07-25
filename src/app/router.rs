use crate::app::App;
use crate::components::page::Page;

pub trait Routeable {
    fn route(app: &mut App) -> Page;
}