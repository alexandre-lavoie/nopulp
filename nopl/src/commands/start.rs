use crate::server::server;
use crate::watch::watch;
use std::thread;

/// Runs the `server()` function simultaneously to the `watch()` function.
pub fn start() {
    thread::spawn(|| server());
    watch().unwrap();
}