extern crate ncurses;

mod window;

#[doc = "The actual user interface"]
pub struct UI {
    map_window: window::Window,
    messages_window: window::Window,
    status_window: window::Window,
}

impl Drop for UI {
    #[stable]
    #[doc = "
        Tidy up everything about the UI when it it destroyed
    "]
    fn drop(&mut self) {
        debug!("Destroying UI");
        ncurses::endwin();
    }
}

impl UI {
    #[stable]
    #[doc = "
    Get the width of the user interface, in cells
    
    # Returns
    The number of cells wide
    "]
    pub fn width(&self) -> uint {
        ncurses::COLS as uint
    }
    #[stable]
    #[doc = "
    Get the height of the user interface, in cells
    
    # Returns
    The number of cells high
    "]
    pub fn height(&self) -> uint {
        ncurses::LINES as uint
    }
}

#[experimental]
#[doc = "
Actually create the UI

# Returns
The object representing the User Interface
"]
pub fn create_ui() -> UI {
    debug!("Creating UI");
    ncurses::initscr();
    UI {
        map_window: window::create_window(),
        messages_window: window::create_window(),
        status_window: window::create_window()
    }
}