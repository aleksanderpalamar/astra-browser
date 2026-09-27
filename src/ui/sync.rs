use gtk::prelude::*;
use gtk::{ApplicationWindow, glib};
use webkit6::WebView;
use webkit6::prelude::*;

use crate::ui::actions::HistoryActions;
use crate::ui::title::window_title;
use crate::ui::toolbar::Toolbar;

pub fn bind(
    webview: &WebView,
    window: &ApplicationWindow,
    toolbar: &Toolbar,
    history: HistoryActions,
) {
    bind_address(webview, toolbar.clone());
    bind_loading(webview, toolbar.clone());
    bind_title(webview, window);
    bind_history(webview, history);
}

fn bind_address(webview: &WebView, toolbar: Toolbar) {
    webview.connect_uri_notify(move |webview| {
        if let Some(uri) = webview.uri() {
            toolbar.show_address(&uri);
        }
    });
}

fn bind_loading(webview: &WebView, toolbar: Toolbar) {
    webview.connect_is_loading_notify(move |webview| toolbar.set_loading(webview.is_loading()));
}

fn bind_title(webview: &WebView, window: &ApplicationWindow) {
    webview.connect_title_notify(glib::clone!(
        #[weak]
        window,
        move |webview| {
            let title = window_title(webview.title().as_deref());
            window.set_title(Some(&title));
        }
    ));
}

fn bind_history(webview: &WebView, history: HistoryActions) {
    history.update(webview);
    let on_load = history.clone();
    webview.connect_load_changed(move |webview, _| on_load.update(webview));
    webview.connect_uri_notify(move |webview| history.update(webview));
}
