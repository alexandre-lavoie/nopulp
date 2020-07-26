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
/// 
/// TODO: Should be moved to a `macros` package.
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
            let mut page = $app::route(&mut app);
            app.render(&mut page);
            App::attach_listeners(app, page);
            Ok(())
        }
    };
}

/// Application router using browser href and matcher.
///
/// Expects external modules that exports the following template:
///
/// ```
/// use nopulp::*;
///
/// pub fn new(app: &mut App) -> Page {
///     nopl! (
///         // Insert page code here!
///     )
/// }
/// ```
///
/// (Previous template is bound to change to use traditional HTML).
///
/// You can use the following template to point to external pages.
///
/// ```
/// pub struct MyApp(App);
///
/// impl Routeable for MyApp {
///     router! {
///         ["path", "to", "page"] => page_1;
///         ["root", "of", "page", ..] => page_2;
///         ["ignore", _, "spot", ..] => page_3;
///         _ => page_default;
///     }
/// }
/// ```
/// 
/// TODO: Should be moved to a `macros` package.
#[macro_export]
macro_rules! router {
    ($($path:pat => $page:ident;)*) => {
        fn route(app: &mut App) -> Page {
            match app.get_href_split().as_slice() {
                $($path => $page::new(app),)*
            }
        }
    };
}

/// Application storage initializer for external pages.
///
/// This is currently bound to the page template use in the [`router!`] macro.
///
/// ```
/// use nopulp::*;
///
/// pub fn new(app: &mut App) -> Page {
///     storage!(
///         app => {
///             "Variable" => "Value"
///         }
///     )
///
///     nopl! (
///         // Insert page code here!
///     )
/// }
/// ```
/// 
/// TODO: Should be moved to a `macros` package.
#[macro_export]
macro_rules! storage {
    ($app:ident => {$($var:literal => $val:expr;)*}) => {
        $($app.storage.insert($var, $val);)*
    };
}
