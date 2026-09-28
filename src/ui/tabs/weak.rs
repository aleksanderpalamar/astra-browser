use gtk::prelude::*;
use gtk::{Notebook, glib};
use webkit6::NetworkSession;

use super::Tabs;
use crate::browser::engine::WebEngine;

pub struct WeakTabs {
    notebook: glib::WeakRef<Notebook>,
    session: glib::WeakRef<NetworkSession>,
    engine: WebEngine,
}

impl glib::clone::Downgrade for Tabs {
    type Weak = WeakTabs;

    fn downgrade(&self) -> WeakTabs {
        WeakTabs {
            notebook: ObjectExt::downgrade(&self.notebook),
            session: ObjectExt::downgrade(&self.session),
            engine: self.engine.clone(),
        }
    }
}

impl glib::clone::Upgrade for WeakTabs {
    type Strong = Tabs;

    fn upgrade(&self) -> Option<Tabs> {
        Some(Tabs {
            notebook: self.notebook.upgrade()?,
            session: self.session.upgrade()?,
            engine: self.engine.clone(),
        })
    }
}
