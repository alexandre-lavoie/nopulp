use crate::app::App;
use crate::core::*;

/// TODO: Should be used to embed content like images into pages.
/// 
/// TODO: Should be moved to a `macros` package.
#[macro_export]
macro_rules! embed {
    ($file:literal) => {
        include_bytes!($file);
    };
}

pub struct Image(pub Object<String>);

impl Child for Image {}

impl Clickable for Image {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Image {
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
