use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use colored::Colorize;
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::build::build;

pub fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

    watcher.watch("./src", RecursiveMode::Recursive)?;

    build();

    println!("{}: Listening for changes...", "[WATCH]".cyan());

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) => {
                    println!("{}: Catched {}", "[WATCH]".cyan(), path.to_str().expect("Path"));
                    build();
                    println!("{}: Listening for changes...", "[WATCH]".cyan());
                },
                _ => ()
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}