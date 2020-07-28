use crate::*;

/// `except` extension that binds to the javascript console.
///
/// Should not be implemented ouside of this file.
pub trait ExpectNopl<T> {
    /// General exception method bound to javascript console.
    fn expectnopl(self, msg: &'static str) -> T;
}

/// Except bound to Option.
impl<T> ExpectNopl<T> for Option<T> {
    /// `Option<T>` exception method bound to javascript console.
    ///
    /// ```
    /// Option<T>.exceptnopl("Error") // Should console.error("Error"); in browser if None.
    /// ```
    fn expectnopl(self, msg: &'static str) -> T {
        match self {
            Some(x) => x,
            None => {
                console::error(msg);
                panic!(msg);
            }
        }
    }
}

/// Except bound to Result.
impl<T, E> ExpectNopl<T> for Result<T, E> {
    /// `Result<T, E>` exception method bound to javascript console.
    ///
    /// ```
    /// Result<T, E>.exceptnopl("Error") // Should console.error("Error"); in browser if Err.
    /// ```
    fn expectnopl(self, msg: &'static str) -> T {
        match self {
            Ok(x) => x,
            Err(_e) => {
                console::error(msg);
                panic!(msg);
            }
        }
    }
}
