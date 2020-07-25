mod commands;

use colored::Colorize;
use std::env;

use crate::commands::*;

fn main() {
    println!("{} - Version 0.1.0\n", "Nopl".magenta());

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        panic!("Missing parameters for commandline.");
    }

    let command = arguments[1].as_str();

    match command {
        "watch" => { watch().expect("Error in command runtime."); },
        "server" => { server(); },
        "build" => { build(); },
        "start" => { start(); },
        _ => panic!("Cannot find command."),
    }
}