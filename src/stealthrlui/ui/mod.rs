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
    fn width(&self) -> uint {
        ncurses::COLS as uint
    }
    #[stable]
    #[doc = "
    Get the height of the user interface, in cells
    
    # Returns
    The number of cells high
    "]
    fn height(&self) -> uint {
        ncurses::LINES as uint
    }
    
    #[stable]
    #[doc = "
    Render any changes that need to be made to the User Interface
    "]
    pub fn render(&self) {
        ncurses::clear();
        ncurses::getch();
    }

    #[experimental]
    #[doc = "
    Actually create the UI
    
    # Returns
    The object representing the User Interface
    "]
    pub fn new() -> UI {
        debug!("Creating UI");
        ncurses::initscr();
        let mut ui = UI {
            windows: TreeMap::new()
        };
        
        let width = ui.width();
        let height = ui.height();
        
        info!("User Interface is {}x{}", ui.width(), ui.height());
        
        let messages_height = 10u;
        let status_width = 20u;
        
        let map_width = width - status_width;
        let map_height = height - messages_height;
        debug!("Messages Height = {}", messages_height);
        debug!("Status Width = {}", status_width);
        debug!("Map Height = {}", map_height);
        debug!("Map Width = {}", map_width);
        
        ui.windows.insert("map".to_string(), window::Window::new(0, 0, map_width, map_height, "Map".to_string()));
        ui.windows.insert("messages".to_string(), window::Window::new(0, map_height, width, messages_height, "Messages".to_string()));
        ui.windows.insert("status".to_string(), window::Window::new(map_width, 0, status_width, map_height, "Status".to_string()));
        
        ui
    }
}
