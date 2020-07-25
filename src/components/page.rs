use crate::core::*;
use crate::app::App;

pub struct Page(pub Object<String>);

impl Child for Page { }

impl Defaultable<Page> for Page {
    fn default() -> Page {
        Page {
            0: Object::default()
        }
    }
}

impl Clickable for Page {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Page {
    fn render(&self, app: &App) {
        self.0.render(app);
    }
}