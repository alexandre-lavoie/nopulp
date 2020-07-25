use crate::core::*;
use crate::app::App;

pub struct Text(pub Object<String>);

impl Child for Text { }

impl Clickable for Text {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Text {
    fn render(&self, app: &App) {
        let context = app.get_context();

        context
            .set_font(self.0.style.font);

        context
            .fill_text(self.0.value.clone().unwrap_or("".to_string()).as_str(), self.0.style.left, self.0.style.top)
            .expectmsk("Should be able to fill text.");

        context
            .stroke();

        self.0.render(app);
    }
}