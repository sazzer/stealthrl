extern crate ncurses;

use std::collections::enum_set::EnumSet;
pub use self::settings::Settings;

mod settings;

#[experimental]
#[doc = "Data Type representing the NCurses system"]
pub struct Ncrs {
    settings: EnumSet<Settings>
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
        let result = Ncrs {
            settings: EnumSet::new()
        };
        result
    }
    
    pub fn update_setting(&mut self, setting: Settings, on: bool) -> &mut Ncrs {
        if self.settings.contains(&setting) != on {
            if on {
                debug!("Enabling setting {}", setting as uint);
                self.settings.insert(setting);
                match setting {
                    Settings::CBREAK => ncurses::cbreak(),
                    Settings::ECHO => ncurses::echo(),
                };
            } else {
                debug!("Disabling setting {}", setting as uint);
                self.settings.remove(&setting);
                match setting {
                    Settings::CBREAK => ncurses::nocbreak(),
                    Settings::ECHO => ncurses::noecho(),
                };
            }
        }
        self
    }
    
    pub fn cbreak(&mut self, on: bool) -> &mut Ncrs {
        self.update_setting(Settings::CBREAK, on);
        self
    }
    
    pub fn echo(&mut self, on: bool) -> &mut Ncrs {
        self.update_setting(Settings::ECHO, on);
        self
    }
}