extern crate nopulp;

use nopulp::*;

mod pages;
use pages::*;

pub struct MyApp;

impl nopulp::Routeable for MyApp {
    router! {
        _ => index;
    }
}

app!(MyApp);