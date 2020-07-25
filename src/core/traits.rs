use crate::app::App;

pub trait Child: Renderable + Clickable { }

pub trait Renderable {
    fn render(&self, app: &App);
}

pub trait Initializable<T, K> {
    fn new(param: T) -> K;
}

pub trait Defaultable<T> {
    fn default() -> T;
}

pub trait Clickable {
    fn on_click(&mut self, app: &mut App);
}