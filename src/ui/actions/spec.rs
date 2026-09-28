use gtk::prelude::*;
use gtk::{Application, gio, glib};

pub trait ActionSpec: Copy + 'static {
    const SCOPE: &'static str;

    fn name(self) -> &'static str;

    fn accels(self) -> &'static [&'static str];

    fn parameter_type(self) -> Option<&'static glib::VariantTy> {
        None
    }

    fn detailed_name(self) -> String {
        format!("{}.{}", Self::SCOPE, self.name())
    }
}

pub fn register<A: ActionSpec>(
    map: &impl IsA<gio::ActionMap>,
    action: A,
    perform: impl Fn(Option<&glib::Variant>) + 'static,
) -> gio::SimpleAction {
    let simple = gio::SimpleAction::new(action.name(), action.parameter_type());
    simple.connect_activate(move |_, parameter| perform(parameter));
    map.add_action(&simple);
    simple
}

pub fn register_toggle<A: ActionSpec>(
    map: &impl IsA<gio::ActionMap>,
    action: A,
    initial: bool,
    changed: impl Fn(bool) + 'static,
) {
    let simple = gio::SimpleAction::new_stateful(action.name(), None, &initial.to_variant());
    simple.connect_change_state(move |simple, state| {
        let Some(enabled) = state.and_then(|state| state.get::<bool>()) else {
            return;
        };
        simple.set_state(&enabled.to_variant());
        changed(enabled);
    });
    map.add_action(&simple);
}

pub fn register_accels<A: ActionSpec>(app: &Application, actions: impl IntoIterator<Item = A>) {
    for action in actions {
        app.set_accels_for_action(&action.detailed_name(), action.accels());
    }
}
