use gtk::prelude::*;
use gtk::{Application, gio, glib};

use crate::library::Library;
use crate::ui::actions::{BrowserAction, LibraryAction, register_accels};
use crate::ui::window;

const APP_ID: &str = "io.github.aleksanderpalamar.RustBrowser";
const QUIT_ACTION: &str = "quit";
const QUIT_ACCELS: &[&str] = &["<Control>q"];

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    let library = Library::open();
    app.connect_startup(|app| {
        install_quit_action(app);
        register_accelerators(app);
    });
    app.connect_activate(move |app| activate(app, &library));
    app.run()
}

fn activate(app: &Application, library: &Library) {
    if let Some(window) = app.active_window() {
        window.present();
        return;
    }
    window::build(app, library);
}

fn install_quit_action(app: &Application) {
    let quit = gio::ActionEntry::builder(QUIT_ACTION)
        .activate(|app: &Application, _, _| app.quit())
        .build();
    app.add_action_entries([quit]);
}

fn register_accelerators(app: &Application) {
    app.set_accels_for_action(&format!("app.{QUIT_ACTION}"), QUIT_ACCELS);
    register_accels(app, BrowserAction::ALL);
    register_accels(app, LibraryAction::ALL);
}

#[cfg(test)]
mod tests {
    use super::QUIT_ACCELS;
    use crate::ui::actions::{ActionSpec, BrowserAction, LibraryAction};

    #[test]
    fn accelerators_are_unique_across_all_actions() {
        let mut accels: Vec<&str> = BrowserAction::ALL
            .into_iter()
            .flat_map(ActionSpec::accels)
            .chain(LibraryAction::ALL.into_iter().flat_map(ActionSpec::accels))
            .chain(QUIT_ACCELS.iter())
            .copied()
            .collect();
        accels.sort_unstable();
        assert!(accels.windows(2).all(|pair| pair[0] != pair[1]));
    }
}
