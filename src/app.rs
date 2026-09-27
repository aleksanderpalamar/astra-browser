use gtk::prelude::*;
use gtk::{Application, glib};

use crate::browser::mode::BrowsingMode;
use crate::library::Library;
use crate::ui::actions::{self, AppAction, BrowserAction, LibraryAction, register_accels};
use crate::ui::window;

const APP_ID: &str = "io.github.aleksanderpalamar.RustBrowser";

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    let library = Library::open();
    app.connect_startup(glib::clone!(
        #[strong]
        library,
        move |app| {
            install_app_actions(app, &library);
            register_accelerators(app);
        }
    ));
    app.connect_activate(move |app| activate(app, &library));
    app.run()
}

fn activate(app: &Application, library: &Library) {
    if let Some(window) = app.active_window() {
        window.present();
        return;
    }
    window::build(app, library, BrowsingMode::Normal);
}

fn install_app_actions(app: &Application, library: &Library) {
    for (action, mode) in [
        (AppAction::NewWindow, BrowsingMode::Normal),
        (AppAction::NewPrivateWindow, BrowsingMode::Private),
    ] {
        actions::register(
            app,
            action,
            glib::clone!(
                #[weak]
                app,
                #[strong]
                library,
                move |_| window::build(&app, &library, mode)
            ),
        );
    }
    actions::register(
        app,
        AppAction::Quit,
        glib::clone!(
            #[weak]
            app,
            move |_| app.quit()
        ),
    );
}

fn register_accelerators(app: &Application) {
    register_accels(app, AppAction::ALL);
    register_accels(app, BrowserAction::ALL);
    register_accels(app, LibraryAction::ALL);
}

#[cfg(test)]
mod tests {
    use crate::ui::actions::{ActionSpec, AppAction, BrowserAction, LibraryAction};

    #[test]
    fn accelerators_are_unique_across_all_actions() {
        let mut accels: Vec<&str> = BrowserAction::ALL
            .into_iter()
            .flat_map(ActionSpec::accels)
            .chain(LibraryAction::ALL.into_iter().flat_map(ActionSpec::accels))
            .chain(AppAction::ALL.into_iter().flat_map(ActionSpec::accels))
            .copied()
            .collect();
        accels.sort_unstable();
        assert!(accels.windows(2).all(|pair| pair[0] != pair[1]));
    }
}
