use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, gio};

use crate::library::preferences::{Preferences, Toggle};
use crate::ui::actions::{ActionSpec, AppAction};

pub fn install(
    app: &Application,
    action: AppAction,
    toggle: Toggle,
    preferences: Rc<Preferences>,
    apply: impl Fn(bool) + 'static,
) {
    let enabled = preferences.enabled(toggle);
    apply(enabled);
    let simple = gio::SimpleAction::new_stateful(action.name(), None, &enabled.to_variant());
    simple.connect_change_state(move |simple, state| {
        let Some(enabled) = state.and_then(|state| state.get::<bool>()) else {
            return;
        };
        simple.set_state(&enabled.to_variant());
        apply(enabled);
        preferences.set_enabled(toggle, enabled);
    });
    app.add_action(&simple);
}
