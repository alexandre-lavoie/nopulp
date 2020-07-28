use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

/// External app definition structure.
///
/// Should not be initialized directly, only used as reference as follow:
///
/// ```
/// pub struct MyApp(App);
/// ```
///
/// You can use the `app!` macro to initialize the app.
#[derive(Clone)]
pub struct App {
    window: web_sys::Window,
    pub storage: HashMap<&'static str, &'static str>,
}

/// External app implementation.
impl App {
    /// Returns the href path for the window bound to the application.
    ///
    /// ```
    /// let app = App::new(()); // Assuming bound to window with href `http://localhost:8080/current/path`.
    ///
    /// assert_eq!(app.get_href(), "/current/path".to_string());
    /// ```
    pub fn get_href(&self) -> String {
        self.window
            .location()
            .pathname()
            .expectnopl("Should be able to get pathname from window.")
    }

    /// Returns the split href path for window bound to the application.
    ///
    /// ```
    /// let app = App::new(()); // Assuming bound to window with href `http://localhost:8080/current/path`.
    ///
    /// assert_eq!(app.get_href_split(), vec!["current".to_string(), "path".to_string()]);
    /// ```
    pub fn get_href_split(&self) -> Vec<&str> {
        Box::leak(self.get_href().into_boxed_str())
            .split("/")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
    }

    /// Returns the body from the bound window.
    ///
    /// ```
    /// let app = App::new(());
    ///
    /// let body = app.get_body(); // Reference to HTML body.
    /// ```
    pub fn get_body(&self) -> web_sys::HtmlElement {
        self.get_document()
            .body()
            .expectnopl("Should be able to get document from window.")
    }

    /// Returns the document from the bound window.
    ///
    /// ```
    /// let app = App::new(());
    ///
    /// let document = app.get_document(); // Reference to HTML document.
    /// ```
    pub fn get_document(&self) -> web_sys::Document {
        self.window
            .document()
            .expectnopl("Should be able to get body from document.")
    }

    /// Returns the canvas bound to the window.
    ///
    /// ```
    /// let app = App::new(());
    ///
    /// let canvas = app.get_canvas(); // Reference to canvas.
    /// ```
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.get_document()
            .get_element_by_id("nopl")
            .expectnopl("Canvas should exist.")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expectnopl("Should be able to create canvas.")
    }

    /// Returns the context from the canvas bound to the window.
    ///
    /// Currently returns context `2d` -> considering to implement `webgl`.
    ///
    /// ```
    /// let app = App::new(());
    ///
    /// let context = app.get_context(); // Reference to canvas context.
    /// ```
    pub fn get_context(&self) -> web_sys::CanvasRenderingContext2d {
        self.get_canvas()
            .get_context("2d")
            .expectnopl("Should be able to get context of canvas.")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expectnopl("Should be able to convert canvas context to context 2d.")
    }

    /// Clear canvas and renders page using current application.
    ///
    /// Should not be used inside external application -> framework should handle using the [`router!`] macro.
    pub fn render(&mut self, html: &mut Html) {
        self.get_context().clear_rect(0f64, 0f64, 1920f64, 1080f64);

        html.render(&self);
    }

    /// Attaches javascript listeners to application.
    ///
    /// Should not be used inside external application -> framework should handle using the [`app!`] macro.
    pub fn attach_listeners(app: App, html: Html) {
        let app = Rc::new(RefCell::new(app));
        let html = Rc::new(RefCell::new(html));

        {
            let app = app.clone();
            let page = html.clone();

            let canvas = app.borrow().get_canvas();

            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                app.clone()
                    .borrow_mut()
                    .on_click(&mut page.clone().borrow_mut());

                app.clone()
                    .borrow_mut()
                    .render(&mut page.clone().borrow_mut());
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .expectnopl("Should be able to bind to mouse down.");
            closure.forget();
        }
    }

    /// Method bound to the javascript `mousedown` event.
    ///
    /// Should find a way to attach it to [`Clickable`].
    fn on_click(&mut self, html: &mut Html) {
        html.on_click(self);
    }
}

impl Initializable<(), App> for App {
    fn new(_: ()) -> App {
        let window = web_sys::window().expectnopl("No global `window` exists.");

        let document = window
            .document()
            .expectnopl("Should be able to get document from window.");

        let body = document
            .body()
            .expectnopl("Should be able to get body from document.");

        body.style()
            .set_property("margin", "0")
            .expectnopl("Should be able to set body margin to zero.");

        let canvas = document
            .create_element("canvas")
            .expectnopl("Canvas should exist.")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expectnopl("Should be able to create canvas.");

        canvas
            .set_attribute("id", "nopl")
            .expectnopl("Should be able to set canvas id to nopulp.");
        canvas
            .set_attribute(
                "width",
                window
                    .inner_width()
                    .expectnopl("Window should have inner width.")
                    .as_f64()
                    .expectnopl("Width should be f64.")
                    .to_string()
                    .as_str(),
            )
            .expectnopl("Should be able to set canvas width.");
        canvas
            .set_attribute(
                "height",
                window
                    .inner_height()
                    .expectnopl("Window should have inner height.")
                    .as_f64()
                    .expectnopl("Height should be f64.")
                    .to_string()
                    .as_str(),
            )
            .expectnopl("Should be able to set canvas height.");

        body.append_child(&canvas)
            .expectnopl("Should be able to append canvas to body.");

        App {
            window,
            storage: HashMap::new(),
        }
    }
}