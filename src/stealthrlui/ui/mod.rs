extern crate ncurses;

use std::collections::TreeMap;
mod window;

#[doc = "The actual user interface"]
pub struct UI {
    windows: TreeMap<String, window::Window>
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
    let mut ui = UI {
        windows: TreeMap::new()
    };
    
    let width = ui.width();
    let height = ui.height();
    
    let messages_height = 10u;
    let status_width = 20u;
    
    let map_width = width - status_width;
    let map_height = height - messages_height;
    
    ui.windows.insert("map".to_string(), window::create_window(0, 0, map_width, map_height, "Map".to_string()));
    ui.windows.insert("messages".to_string(), window::create_window(0, map_height, width, messages_height, "Messages".to_string()));
    ui.windows.insert("status".to_string(), window::create_window(map_width, 0, status_width, map_height, "Status".to_string()));
    
    ui
}