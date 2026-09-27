use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use webkit6::WebView;

use super::Tabs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabProperty {
    Uri,
    Title,
    Loading,
}

impl TabProperty {
    fn name(self) -> &'static str {
        match self {
            Self::Uri => "uri",
            Self::Title => "title",
            Self::Loading => "is-loading",
        }
    }
}

impl Tabs {
    pub fn watch_current<F: Fn(&WebView) + 'static>(
        &self,
        properties: &'static [TabProperty],
        callback: F,
    ) {
        let callback = Rc::new(callback);
        let on_change = Rc::clone(&callback);
        self.connect_tab_added(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            move |webview| {
                for property in properties {
                    let on_change = Rc::clone(&on_change);
                    webview.connect_notify_local(
                        Some(property.name()),
                        glib::clone!(
                            #[weak]
                            tabs,
                            move |webview, _| {
                                if tabs.is_current(webview) {
                                    on_change(webview);
                                }
                            }
                        ),
                    );
                }
            }
        ));
        self.connect_tab_selected(move |webview| callback(webview));
    }
}
