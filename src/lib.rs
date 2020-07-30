///! Framework for making compiled websites.
///!
///! Nopulp is a framework create web application 100% in Rust.
///!
///! # Quick Start
///!
///! You can use `nopl` CLI to create a default package. - Still work in progress.
///! Refer to example for syntax.
#[doc()]

pub use nopulp_components::*;
pub use nopulp_core::{app, router, storage, embed, App, Html, Object, Routeable, Style, Initializable};
pub use nopulp_html::{html};