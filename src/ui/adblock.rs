use std::rc::Rc;

use gtk::{Application, glib};

use crate::browser::adblock::AdBlocker;
use crate::library::preferences::Preferences;
use crate::ui::actions::{self, AppAction};

pub fn install(app: &Application, adblocker: &AdBlocker, preferences: Rc<Preferences>) {
    let enabled = preferences.adblock_enabled();
    adblocker.set_enabled(enabled);
    actions::register_toggle(
        app,
        AppAction::ToggleAdBlock,
        enabled,
        glib::clone!(
            #[strong]
            adblocker,
            move |enabled| {
                adblocker.set_enabled(enabled);
                preferences.set_adblock_enabled(enabled);
            }
        ),
    );
}
