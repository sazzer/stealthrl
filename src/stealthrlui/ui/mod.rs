
#[doc = "The actual user interface"]
pub struct UI;

impl Drop for UI {
    #[stable]
    #[doc = "
        Tidy up everything about the UI when it it destroyed
    "]
    fn drop(&mut self) {
        debug!("Destroying UI");
    }
}

impl UI {
}

#[experimental]
#[doc = "
Actually create the UI

# Returns
The object representing the User Interface
"]
pub fn create_ui() -> UI {
    debug!("Creating UI");
    UI
}