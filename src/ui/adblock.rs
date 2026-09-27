use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, gio, glib};

use crate::browser::adblock::AdBlocker;
use crate::library::preferences::Preferences;
use crate::ui::actions::{ActionSpec, AppAction};

pub fn install(app: &Application, adblocker: &AdBlocker, preferences: Rc<Preferences>) {
    let enabled = preferences.adblock_enabled();
    adblocker.set_enabled(enabled);
    let action = gio::SimpleAction::new_stateful(
        AppAction::ToggleAdBlock.name(),
        None,
        &enabled.to_variant(),
    );
    action.connect_change_state(glib::clone!(
        #[strong]
        adblocker,
        move |action, state| {
            let Some(enabled) = state.and_then(|state| state.get::<bool>()) else {
                return;
            };
            action.set_state(&enabled.to_variant());
            adblocker.set_enabled(enabled);
            preferences.set_adblock_enabled(enabled);
        }
    ));
    app.add_action(&action);
}
