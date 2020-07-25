use crate::core::*;
use crate::app::App;

pub struct Object<T> where T: Clone {
    pub style: Style<T>,
    pub value: Option<T>,
    pub children: Vec<Box<dyn Child>>
}

impl<T> Defaultable<Object<T>> for Object<T> where T: Clone {
    fn default() -> Object<T> {
        Object {
            style: Style::default(),
            value: None,
            children: vec![],
        }
    }
}

impl<T> Renderable for Object<T> where T: Clone {
    fn render(&self, app: &App) {
        for child in &self.children {
            child.render(app);
        }
    }
}

impl<T> Clickable for Object<T> where T: Clone {
    fn on_click(&mut self, app: &mut App) {
        let style: Style<T> = self.style.clone();

        (style.on_click)(app, self);

        for child in &mut self.children {
            child.on_click(app);
        }
    }
}