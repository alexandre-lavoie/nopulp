use crate::core::*;

pub trait ExpectMsk<T> {
    fn expectmsk(self, msg: &'static str) -> T;
}

impl <T> ExpectMsk<T> for Option<T> {
    fn expectmsk(self, msg: &'static str) -> T {
        match self {
            Some(x) => x,
            None => {
                console::error(msg);
                panic!(msg);
            }
        }
    }
}

impl <T, E> ExpectMsk<T> for Result<T, E> {
    fn expectmsk(self, msg: &'static str) -> T {
        match self {
            Ok(x) => x,
            Err(_e) => {
                console::error(msg);
                panic!(msg);
            }
        }
    }
}