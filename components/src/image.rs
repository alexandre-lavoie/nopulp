use nopulp_core::*;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Image(pub Object<&'static [u8]>);

impl Child for Image {}

impl Clickable for Image {
    fn on_click(&mut self, app: &mut App) {
        self.0.on_click(app);
    }
}

impl Renderable for Image {
    fn render(&self, app: &App) {
        let context = app.get_context();

        let image = web_sys::HtmlImageElement::new().unwrap();

        image.set_src(format!("data:image/png;base64,{}", base64::encode(self.0.value.unwrap())).as_str());

        image.set_width(self.0.style.width as u32);

        image.set_height(self.0.style.height as u32);

        context.draw_image_with_html_image_element(&image, self.0.style.left, self.0.style.top).unwrap();

        context.stroke();

        self.0.render(app);
    }
}
