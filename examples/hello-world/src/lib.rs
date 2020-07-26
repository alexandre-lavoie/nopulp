#[macro_use]
extern crate nopulp_html;

#[macro_use]
extern crate nopulp;

use nopulp::*;

mod pages;
use pages::*;

pub struct MyApp;

router! {
    MyApp => {
        _ => html_page;
    }
}

app!(MyApp);