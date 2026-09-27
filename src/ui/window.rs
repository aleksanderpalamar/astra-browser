use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, glib};
use webkit6::NetworkSession;

use std::rc::Rc;

use crate::browser::navigation::HOME_URI;
use crate::library::Library;
use crate::ui::tabs::Tabs;
use crate::ui::title::APP_NAME;
use crate::ui::toolbar::Toolbar;
use crate::ui::{actions, bookmarks, downloads, history, sync};

const DEFAULT_WIDTH: i32 = 1280;
const DEFAULT_HEIGHT: i32 = 800;

pub fn build(app: &Application, library: &Library) {
    let tabs = Tabs::new();
    let toolbar = Toolbar::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&layout(&toolbar, &tabs))
        .build();
    let back_forward = actions::install(&window, &tabs, &toolbar);
    sync::bind(&tabs, &window, &toolbar, back_forward);
    bookmarks::install(&window, &tabs, &toolbar, Rc::clone(&library.bookmarks));
    history::install(&window, &tabs, &toolbar, Rc::clone(&library.history));
    install_downloads(&window, &toolbar);
    release_tabs_on_close(&window, tabs.clone());
    tabs.open(HOME_URI);
    window.present();
}

fn layout(toolbar: &Toolbar, tabs: &Tabs) -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.append(toolbar.widget());
    content.append(tabs.widget());
    content
}

fn install_downloads(window: &ApplicationWindow, toolbar: &Toolbar) {
    match NetworkSession::default() {
        Some(session) => downloads::install(window, toolbar, &session),
        None => eprintln!("Sessão de rede padrão indisponível; downloads desativados"),
    }
}

fn release_tabs_on_close(window: &ApplicationWindow, tabs: Tabs) {
    window.connect_close_request(move |_| {
        tabs.close_all();
        glib::Propagation::Proceed
    });
}
