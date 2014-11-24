extern crate ncurses;

#[experimental]
#[doc = "Data Type representing the NCurses system"]
pub struct Ncrs;

impl Drop for Ncrs {
    #[stable]
    #[doc = "Shut down NCurses"]
    fn drop(&mut self) {
        debug!("Destroying NCRS");
        ncurses::endwin();
    }
}

impl Ncrs {
    #[experimental]
    #[doc = "Create a new NCRS system."]
    pub fn new() -> Ncrs {
        debug!("Creating a new NCRS");
        ncurses::initscr();
        Ncrs
    }
}