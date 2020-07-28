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