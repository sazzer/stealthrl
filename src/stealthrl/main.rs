#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate stealthrlui;

#[cfg(not(test))]
fn main() {
    info!("Starting...");
    let ui = stealthrlui::ui::UI::new();
    ui.render();
    info!("Stopping...");
}