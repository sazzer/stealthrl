pub struct UI;

impl Drop for UI {
    fn drop(&mut self) {
        debug!("Destroying UI");
    }
}

impl UI {
}

pub fn create_ui() -> UI {
    debug!("Creating UI");
    UI
}