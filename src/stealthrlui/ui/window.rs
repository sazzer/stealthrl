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
    pub fn new(x: uint, y: uint, width: uint, height: uint, name: String) -> Window {
        debug!("Creating Window {} at ({},{}) with size ({},{})", name, x, y, width, height);
        Window { x: x, 
            y: y, 
            width: width, 
            height: height, 
            name: name
        }
    }
}

