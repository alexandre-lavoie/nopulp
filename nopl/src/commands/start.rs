use crate::server::server;
use crate::watch::watch;
use std::thread;

pub fn start() {
    thread::spawn(|| server());
    watch().unwrap();
}