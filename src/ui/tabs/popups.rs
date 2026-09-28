use gtk::prelude::*;
use gtk::{Window, glib};
use webkit6::prelude::*;
use webkit6::{NavigationAction, WebView};

use super::Tabs;
use crate::browser::popup::{self, Geometry, Opening, Presentation};
use crate::ui::popup_window;

impl Tabs {
    pub(super) fn open_popups(&self, opener: &WebView) {
        opener.connect_create(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            #[upgrade_or_default]
            move |opener, action| {
                if !action.is_user_gesture() {
                    return None;
                }
                Some(tabs.create_popup(opener, action).upcast())
            }
        ));
    }

    fn create_popup(&self, opener: &WebView, action: &NavigationAction) -> WebView {
        let popup = self.webviews.create_related(opener);
        let opening = Opening::from_navigation(action.navigation_type());
        let opener = ObjectExt::downgrade(opener);
        popup.connect_ready_to_show(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            move |popup| tabs.show_popup(popup, opener.upgrade().as_ref(), opening)
        ));
        popup
    }

    fn show_popup(&self, popup: &WebView, opener: Option<&WebView>, opening: Opening) {
        let requested = requested_geometry(popup);
        let default_size = opener.and_then(opener_default_size);
        let presentation = match (requested, default_size) {
            (Some(requested), Some(default_size)) => {
                popup::presentation(opening, requested, default_size)
            }
            _ => Presentation::Tab,
        };
        match presentation {
            Presentation::Tab => self.add_popup_tab(popup, opener),
            Presentation::Window { width, height } => {
                popup_window::present(popup, opener, width, height)
            }
        }
    }

    fn add_popup_tab(&self, popup: &WebView, opener: Option<&WebView>) {
        self.add(popup);
        let opener = opener.map(ObjectExt::downgrade);
        popup.connect_close(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            move |popup| {
                tabs.close(popup);
                if let Some(opener) = opener.as_ref().and_then(|opener| opener.upgrade()) {
                    tabs.show(&opener);
                }
            }
        ));
    }
}

fn requested_geometry(popup: &WebView) -> Option<Geometry> {
    let geometry = popup.window_properties()?.geometry();
    Some(Geometry {
        x: geometry.x(),
        y: geometry.y(),
        width: geometry.width(),
        height: geometry.height(),
    })
}

fn opener_default_size(opener: &WebView) -> Option<(i32, i32)> {
    opener
        .root()
        .and_downcast::<Window>()
        .map(|window| window.default_size())
}
