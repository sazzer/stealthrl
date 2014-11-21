extern crate ncurses;

pub struct Window {
    x: uint,
    y: uint,
    width: uint,
    height: uint,
    name: String,
    window: ncurses::WINDOW,
}

impl Drop for Window {
    #[stable]
    #[doc = "
        Tidy up everything about the Window when it it destroyed
    "]
    fn drop(&mut self) {
        debug!("Destroying Window");
        ncurses::delwin(self.window);
    }
}

impl Window {
    pub fn render(&self) {
        debug!("Rendering window {}", self.name);
        ncurses::mvwprintw(self.window, 1, 1, "Hello");
        ncurses::wnoutrefresh(self.window);
    }
    
    pub fn new(x: uint, y: uint, width: uint, height: uint, name: String) -> Window {
        debug!("Creating Window {} at ({},{}) with size ({},{})", name, x, y, width, height);
        
        let window = ncurses::derwin(ncurses::stdscr, height as i32, width as i32, y as i32, x as i32);
        
        Window { x: x, 
            y: y, 
            width: width, 
            height: height, 
            name: name,
            window: window
        }
    }
}

