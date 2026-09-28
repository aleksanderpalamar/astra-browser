use gtk::prelude::*;
use gtk::{Notebook, glib};
use webkit6::{NetworkSession, UserContentManager};

use super::Tabs;

pub struct WeakTabs {
    notebook: glib::WeakRef<Notebook>,
    session: glib::WeakRef<NetworkSession>,
    content: glib::WeakRef<UserContentManager>,
}

impl glib::clone::Downgrade for Tabs {
    type Weak = WeakTabs;

    fn downgrade(&self) -> WeakTabs {
        WeakTabs {
            notebook: ObjectExt::downgrade(&self.notebook),
            session: ObjectExt::downgrade(&self.session),
            content: ObjectExt::downgrade(&self.content),
        }
    }
}

impl glib::clone::Upgrade for WeakTabs {
    type Strong = Tabs;

    fn upgrade(&self) -> Option<Tabs> {
        Some(Tabs {
            notebook: self.notebook.upgrade()?,
            session: self.session.upgrade()?,
            content: self.content.upgrade()?,
        })
    }
}
