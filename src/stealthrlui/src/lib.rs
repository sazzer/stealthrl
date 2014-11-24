extern crate ncrs;

pub fn stealthrlui() {
	let mut ui = ncrs::Ncrs::new();
	ui.cbreak(false)
	    .cbreak(true)
	    .cbreak(true)
	    .cbreak(false);
}
