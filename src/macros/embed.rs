/// TODO: Should be used to embed content like images into pages.
#[macro_export]
macro_rules! embed {
    ($file:literal) => {
        include_bytes!($file);
    };
}