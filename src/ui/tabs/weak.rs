use gtk::prelude::*;
use gtk::{Notebook, glib};
use webkit6::NetworkSession;

use super::Tabs;
use crate::browser::webview::WebViewFactory;

pub struct WeakTabs {
    notebook: glib::WeakRef<Notebook>,
    session: glib::WeakRef<NetworkSession>,
    webviews: WebViewFactory,
}

impl glib::clone::Downgrade for Tabs {
    type Weak = WeakTabs;

    fn downgrade(&self) -> WeakTabs {
        WeakTabs {
            notebook: ObjectExt::downgrade(&self.notebook),
            session: ObjectExt::downgrade(&self.session),
            webviews: self.webviews.clone(),
        }
    }
}

impl glib::clone::Upgrade for WeakTabs {
    type Strong = Tabs;

    fn upgrade(&self) -> Option<Tabs> {
        Some(Tabs {
            notebook: self.notebook.upgrade()?,
            session: self.session.upgrade()?,
            webviews: self.webviews.clone(),
        })
    }
}
