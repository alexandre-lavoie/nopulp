/// Application compiler macro for external applications.
///
/// Require that you implement the [`Routeable`] trait on your application.
///
/// ```
/// pub struct MyApp(App);
///
/// // Insert router config here.
///
/// app!(MyApp);
/// ```
#[macro_export]
macro_rules! app {
    ($app: ident) => {
        use wasm_bindgen::prelude::*;

        #[cfg(feature = "wee_alloc")]
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        #[wasm_bindgen(start)]
        pub fn run() -> Result<(), JsValue> {
            let mut app = App::new(());
            let mut page: Html = *$app::route(&mut app);
            app.render(&mut page);
            App::attach_listeners(app, page);
            Ok(())
        }
    };
}