use crate::*;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Html(pub Object<String>);

impl Child for Html {}

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
