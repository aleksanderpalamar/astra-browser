use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use webkit6::WebView;

use crate::browser::navigation::Navigator;
use crate::browser::webview;
use crate::ui::title::APP_NAME;
use crate::ui::toolbar::Toolbar;
use crate::ui::{actions, sync};

const DEFAULT_WIDTH: i32 = 1280;
const DEFAULT_HEIGHT: i32 = 800;

pub fn build(app: &Application) {
    let webview = webview::create();
    let toolbar = Toolbar::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&layout(&toolbar, &webview))
        .build();
    let navigator = Navigator::new(webview.clone());
    let history = actions::install(&window, navigator.clone(), toolbar.clone());
    sync::bind(&webview, &window, &toolbar, history);
    navigator.home();
    window.present();
}

fn layout(toolbar: &Toolbar, webview: &WebView) -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.append(toolbar.widget());
    content.append(webview);
    content
}
