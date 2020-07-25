use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod router;
pub mod macros;
use crate::core::*;
pub use router::*;
use crate::components::page::Page;

#[derive(Clone)]
pub struct App {
    window: web_sys::Window,
    pub storage: HashMap<&'static str, &'static str>
}

impl App {
    pub fn get_href(&self) -> String {
        self.window
            .location()
            .pathname()
            .expectmsk("Should be able to get pathname from window.")
    }

    pub fn get_href_split(&self) -> Vec<&str> {
        Box::leak(self.get_href().into_boxed_str())
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
    }

    pub fn get_body(&self) -> web_sys::HtmlElement {
        self.get_document()
            .body()
            .expectmsk("Should be able to get document from window.")
    }

    pub fn get_document(&self) -> web_sys::Document {
        self.window
            .document()
            .expectmsk("Should be able to get body from document.")
    }

    pub fn get_context(&self) -> web_sys::CanvasRenderingContext2d {
        self.get_canvas()
            .get_context("2d")
            .expectmsk("Should be able to get context of canvas.")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expectmsk("Should be able to convert canvas context to context 2d.")
    }

    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.get_document()
            .get_element_by_id("nopl")
            .expectmsk("Canvas should exist.")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expectmsk("Should be able to create canvas.")
    }

    pub fn render(&mut self, page: &mut Page) {
        self.get_context().clear_rect(0f64, 0f64, 1920f64, 1080f64);
        page.render(&self);
    }

    pub fn attach_listeners(app: App, page: Page) {
        let app = Rc::new(RefCell::new(app));
        let page = Rc::new(RefCell::new(page));

        {
            let app = app.clone();
            
            let page = page.clone();

            let canvas = app.borrow().get_canvas();

            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                app.clone().borrow_mut().on_click(&mut page.clone().borrow_mut());

                app.clone().borrow_mut().render(&mut page.clone().borrow_mut());
            }) as Box<dyn FnMut(_)>);
    
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).expectmsk("Should be able to bind to mouse down.");
            
            closure.forget();
        }
    }

    pub fn on_click(&mut self, page: &mut Page) {
        page.on_click(self);
    }
}

impl Initializable<(), App> for App {
    fn new(_: ()) -> App {
        let window = web_sys::window()
            .expect("No global `window` exists.");

        let document = window
            .document()
            .expectmsk("Should be able to get document from window.");

        let body = document
            .body()
            .expectmsk("Should be able to get body from document.");

        body
            .style()
            .set_property("margin", "0")
            .expectmsk("Should be able to set body margin to zero.");

        let canvas = document
            .create_element("canvas")
            .expectmsk("Canvas should exist.")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expectmsk("Should be able to create canvas.");

        canvas
            .set_attribute("id", "nopl")
            .expectmsk("Should be able to set canvas id to nopulp.");
    
        canvas
            .set_attribute("width", window
                .inner_width()
                .expectmsk("Window should have inner width.")
                .as_f64()
                .expectmsk("Width should be f64.")
                .to_string()
                .as_str()
            )
            .expectmsk("Should be able to set canvas width.");
    
        canvas
            .set_attribute("height", window
                .inner_height()
                .expectmsk("Window should have inner height.")
                .as_f64()
                .expectmsk("Height should be f64.")
                .to_string()
                .as_str()
            )
            .expectmsk("Should be able to set canvas height.");

        body
            .append_child(&canvas)
            .expectmsk("Should be able to append canvas to body.");

        App { 
            window,
            storage: HashMap::new()
        }
    }
}