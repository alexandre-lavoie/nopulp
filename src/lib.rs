///! Framework for making compiled websites.
///! 
///! Nopulp is a framework create web application 100% in Rust.
///! 
///! # Quick Start
///!
///! You can use `nopl` CLI to create a default package. - Still work in progress.
///! Refer to example for syntax. 
#[doc()]

pub mod core;
pub use crate::core::*;
pub mod components;
pub use components::*;
pub mod app;
pub use app::*;
pub mod languages;