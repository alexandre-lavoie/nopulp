use crate::app::App;
use crate::core::*;

pub struct Html(pub Object<String>);

impl Child for Html {}

impl Defaultable<Html> for Html {
    fn default() -> Self {
        Html {
            0: Object::default(),
        }
    }
}

impl Clickable for Html {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Html {
    fn render(&self, app: &App) {
        self.0.render(app);
    }
}
