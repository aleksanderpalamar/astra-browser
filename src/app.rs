use gtk::prelude::*;
use gtk::{Application, gio, glib};

use crate::ui::actions::BrowserAction;
use crate::ui::window;

const APP_ID: &str = "io.github.aleksanderpalamar.RustBrowser";
const QUIT_ACTION: &str = "quit";
const QUIT_ACCELS: &[&str] = &["<Control>q"];

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|app| {
        install_quit_action(app);
        register_accelerators(app);
    });
    app.connect_activate(activate);
    app.run()
}

fn activate(app: &Application) {
    if let Some(window) = app.active_window() {
        window.present();
        return;
    }
    window::build(app);
}

fn install_quit_action(app: &Application) {
    let quit = gio::ActionEntry::builder(QUIT_ACTION)
        .activate(|app: &Application, _, _| app.quit())
        .build();
    app.add_action_entries([quit]);
}

fn register_accelerators(app: &Application) {
    app.set_accels_for_action(&format!("app.{QUIT_ACTION}"), QUIT_ACCELS);
    for action in BrowserAction::ALL {
        app.set_accels_for_action(&action.detailed_name(), action.accels());
    }
}
