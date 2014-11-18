#![feature(phase)]
#[phase(plugin, link)] extern crate log;

#[cfg(not(test))]
fn main() {
    info!("Starting...");
    info!("Stopping...");
}