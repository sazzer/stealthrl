pub struct Window {
    x: uint,
    y: uint,
    width: uint,
    height: uint,
    name: String,
}

impl Drop for Window {
    #[stable]
    #[doc = "
        Tidy up everything about the Window when it it destroyed
    "]
    fn drop(&mut self) {
        debug!("Destroying Window");
    }
}

impl Window {
}

pub fn create_window() -> Window {
    debug!("Creating Window");
    Window { x: 0, 
        y: 1, 
        width: 2, 
        height: 3, 
        name: "Window".to_string()
    }
}