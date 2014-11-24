#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate stealthrlui;

#[cfg(not(test))]
fn main() {
    info!("Starting...");
stealthrlui::stealthrlui();
    info!("Stopping...");
}

