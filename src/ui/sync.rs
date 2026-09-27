use gtk::prelude::*;
use gtk::{ApplicationWindow, glib};
use webkit6::prelude::*;

use crate::ui::actions::BackForwardActions;
use crate::ui::tabs::{TabProperty, Tabs};
use crate::ui::title::window_title;
use crate::ui::toolbar::Toolbar;

pub fn bind(
    tabs: &Tabs,
    window: &ApplicationWindow,
    toolbar: &Toolbar,
    back_forward: BackForwardActions,
) {
    tabs.watch_current(
        &[TabProperty::Uri],
        glib::clone!(
            #[weak]
            toolbar,
            move |webview| toolbar.show_address(webview.uri().as_deref().unwrap_or_default())
        ),
    );
    tabs.watch_current(
        &[TabProperty::Loading],
        glib::clone!(
            #[weak]
            toolbar,
            move |webview| toolbar.set_loading(webview.is_loading())
        ),
    );
    tabs.watch_current(
        &[TabProperty::Title],
        glib::clone!(
            #[weak]
            window,
            move |webview| window.set_title(Some(&window_title(webview.title().as_deref())))
        ),
    );
    tabs.watch_current(&[TabProperty::Uri, TabProperty::Loading], move |webview| {
        back_forward.update(webview);
    });
}
