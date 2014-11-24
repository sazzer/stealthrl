extern crate ncrs;

pub fn stealthrlui() {
	let mut ui = ncrs::Ncrs::new();
	ui.cbreak(false);
	ui.cbreak(true);
	ui.cbreak(true);
	ui.cbreak(false);
}
