use crate::app::App;
use crate::core::*;

pub struct Button(pub Object<String>);

impl Child for Button {}

impl Clickable for Button {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Button {
    fn render(&self, app: &App) {
        let context = app.get_context();

        context.rect(
            self.0.style.left,
            self.0.style.top,
            self.0.style.width,
            self.0.style.height,
        );

        context.stroke();

        self.0.render(app);
    }
}
