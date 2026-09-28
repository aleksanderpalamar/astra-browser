use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Image, Label, Orientation, glib};
use webkit6::UserContentManager;

use crate::browser::mode::BrowsingMode;
use crate::browser::navigation::HOME_URI;
use crate::library::Library;
use crate::ui::tabs::Tabs;
use crate::ui::title::window_title;
use crate::ui::toolbar::Toolbar;
use crate::ui::{actions, bookmarks, downloads, history, menu, permissions, sync};

const DEFAULT_WIDTH: i32 = 1280;
const DEFAULT_HEIGHT: i32 = 800;
const BADGE_SPACING: i32 = 4;
const PRIVATE_TOOLTIP: &str =
    "Navegação privada: histórico, cookies e cache desta janela não são guardados";

pub fn build(
    app: &Application,
    library: &Library,
    content: &UserContentManager,
    mode: BrowsingMode,
) {
    let Some(session) = mode.network_session() else {
        eprintln!("Sessão de rede indisponível; a janela não foi aberta");
        return;
    };
    let tabs = Tabs::new(session, content.clone());
    let toolbar = Toolbar::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(window_title(None, mode))
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&layout(&toolbar, &tabs))
        .build();
    install_features(&window, &tabs, &toolbar, library, mode);
    release_tabs_on_close(&window, tabs.clone());
    tabs.open(HOME_URI);
    window.present();
}

fn install_features(
    window: &ApplicationWindow,
    tabs: &Tabs,
    toolbar: &Toolbar,
    library: &Library,
    mode: BrowsingMode,
) {
    let back_forward = actions::install(window, tabs, toolbar);
    sync::bind(tabs, window, toolbar, back_forward, mode);
    bookmarks::install(window, tabs, toolbar, Rc::clone(&library.bookmarks));
    history::install(window, tabs, toolbar, Rc::clone(&library.history), mode);
    downloads::install(window, toolbar, tabs.session());
    permissions::install(tabs);
    if mode == BrowsingMode::Private {
        toolbar.add_end(&private_badge());
    }
    toolbar.add_end(&menu::button());
}

fn layout(toolbar: &Toolbar, tabs: &Tabs) -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.append(toolbar.widget());
    content.append(tabs.widget());
    content
}

fn private_badge() -> GtkBox {
    let badge = GtkBox::new(Orientation::Horizontal, BADGE_SPACING);
    badge.set_tooltip_text(Some(PRIVATE_TOOLTIP));
    badge.append(&Image::from_icon_name("view-conceal-symbolic"));
    badge.append(&Label::new(Some("Privado")));
    badge
}

fn release_tabs_on_close(window: &ApplicationWindow, tabs: Tabs) {
    window.connect_close_request(move |_| {
        tabs.close_all();
        glib::Propagation::Proceed
    });
}
