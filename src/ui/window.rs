use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, glib};

use crate::browser::navigation::HOME_URI;
use crate::ui::tabs::Tabs;
use crate::ui::title::APP_NAME;
use crate::ui::toolbar::Toolbar;
use crate::ui::{actions, sync};

const DEFAULT_WIDTH: i32 = 1280;
const DEFAULT_HEIGHT: i32 = 800;

pub fn build(app: &Application) {
    let tabs = Tabs::new();
    let toolbar = Toolbar::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&layout(&toolbar, &tabs))
        .build();
    let history = actions::install(&window, tabs.clone(), toolbar.clone());
    sync::bind(&tabs, &window, &toolbar, history);
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

fn release_tabs_on_close(window: &ApplicationWindow, tabs: Tabs) {
    window.connect_close_request(move |_| {
        tabs.close_all();
        glib::Propagation::Proceed
    });
}
