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
#[macro_export]
macro_rules! router {
    ($app: ident => {$($path:pat => $page:ident;)*}) => {
        impl nopulp::Routeable for $app {
            fn route(app: &mut App) -> Box<Html> {
                match app.get_href_split().as_slice() {
                    $($path => $page::new(app),)*
                }
            }
        }
    };
}