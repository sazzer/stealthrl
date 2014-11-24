#![feature(phase)]
#[phase(plugin, link)] extern crate log;

pub use ncrs::Ncrs;

mod ncrs;
