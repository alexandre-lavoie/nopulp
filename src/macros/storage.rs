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
#[macro_export]
macro_rules! storage {
    ($app:ident => {$($var:literal => $val:expr;)*}) => {
        $($app.storage.insert($var, $val);)*
    };
}