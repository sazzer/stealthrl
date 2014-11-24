extern crate ncurses;

#[experimental]
#[doc = "Data Type representing the NCurses system"]
pub struct Ncrs {
    cbreak: bool,
}

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
        ncurses::nocbreak();
        Ncrs {
            cbreak: false
        }
    }
    
    #[experimental]
    #[doc = "Set the CBreak mode to On or Off"]
    pub fn cbreak(&mut self, on: bool) {
        if self.cbreak != on {
            if on {
                debug!("Enabling CBreak mode");
                ncurses::cbreak();
            } else {
                debug!("Disabling CBreak mode");
                ncurses::nocbreak();
            }
            self.cbreak = on;
        }
    }
}