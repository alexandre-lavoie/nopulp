use crate::app::App;
use crate::core::*;

pub struct Span(pub Object<String>);

impl Child for Span {}

impl Clickable for Span {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Span {
    fn render(&self, app: &App) {
        let context = app.get_context();

        context.set_font(self.0.style.font);

        context
            .fill_text(
                self.0.value.clone().unwrap_or("".to_string()).as_str(),
                self.0.style.left,
                self.0.style.top,
            )
            .expectnopl("Should be able to fill span.");

        context.stroke();

        self.0.render(app);
    }
}
