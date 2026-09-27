use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, glib};
use webkit6::{NetworkSession, UserContentManager};

use crate::browser::adblock::AdBlocker;
use crate::browser::mode::BrowsingMode;
use crate::browser::session;
use crate::library::{Library, files};
use crate::ui::actions::{self, AppAction, BrowserAction, LibraryAction, register_accels};
use crate::ui::{adblock, window};

const APP_ID: &str = "io.github.aleksanderpalamar.AstraBrowser";
const FILTERS_DIRECTORY: &str = "content-filters";

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    let library = Library::open();
    let adblocker = AdBlocker::new(&files::data_path(FILTERS_DIRECTORY));
    app.connect_startup(glib::clone!(
        #[strong]
        library,
        #[strong]
        adblocker,
        move |app| {
            gtk::Window::set_default_icon_name(APP_ID);
            persist_default_session_cookies();
            adblock::install(app, &adblocker, Rc::clone(&library.preferences));
            install_app_actions(app, &library, adblocker.content_manager());
            register_accelerators(app);
        }
    ));
    app.connect_activate(move |app| activate(app, &library, adblocker.content_manager()));
    app.run()
}

fn activate(app: &Application, library: &Library, content: &UserContentManager) {
    if let Some(window) = app.active_window() {
        window.present();
        return;
    }
    window::build(app, library, content, BrowsingMode::Normal);
}

fn persist_default_session_cookies() {
    match NetworkSession::default() {
        Some(default_session) => session::persist_cookies(&default_session),
        None => eprintln!("Sessão de rede padrão indisponível; os cookies não serão salvos"),
    }
}

fn install_app_actions(app: &Application, library: &Library, content: &UserContentManager) {
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
                #[strong]
                content,
                move |_| window::build(&app, &library, &content, mode)
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
