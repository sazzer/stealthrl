#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate stealthrlui;

#[cfg(not(test))]
fn main() {
    info!("Starting...");
    let ui = stealthrlui::ui::create_ui();
    info!("User Interface is {}x{}", ui.width(), ui.height());
    info!("Stopping...");
}