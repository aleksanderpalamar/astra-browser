use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Entry, Orientation, Spinner, Widget, glib};

use crate::library::bookmarks::BookmarkState;
use crate::ui::actions::{ActionSpec, BrowserAction};
use crate::ui::address_bar;

const SPACING: i32 = 6;

struct NavigationButton {
    action: BrowserAction,
    icon: &'static str,
    tooltip: &'static str,
}

const NAVIGATION_BUTTONS: [NavigationButton; 4] = [
    NavigationButton {
        action: BrowserAction::Back,
        icon: "go-previous-symbolic",
        tooltip: "Voltar (Alt+←)",
    },
    NavigationButton {
        action: BrowserAction::Forward,
        icon: "go-next-symbolic",
        tooltip: "Avançar (Alt+→)",
    },
    NavigationButton {
        action: BrowserAction::Reload,
        icon: "view-refresh-symbolic",
        tooltip: "Recarregar (Ctrl+R)",
    },
    NavigationButton {
        action: BrowserAction::Home,
        icon: "go-home-symbolic",
        tooltip: "Página inicial (Alt+Home)",
    },
];

impl NavigationButton {
    fn build(&self) -> Button {
        Button::builder()
            .icon_name(self.icon)
            .tooltip_text(self.tooltip)
            .action_name(self.action.detailed_name())
            .build()
    }
}

#[derive(Clone)]
pub struct Toolbar {
    container: GtkBox,
    address: Entry,
    spinner: Spinner,
}

pub struct WeakToolbar {
    container: glib::WeakRef<GtkBox>,
    address: glib::WeakRef<Entry>,
    spinner: glib::WeakRef<Spinner>,
}

impl glib::clone::Downgrade for Toolbar {
    type Weak = WeakToolbar;

    fn downgrade(&self) -> WeakToolbar {
        WeakToolbar {
            container: ObjectExt::downgrade(&self.container),
            address: ObjectExt::downgrade(&self.address),
            spinner: ObjectExt::downgrade(&self.spinner),
        }
    }
}

impl glib::clone::Upgrade for WeakToolbar {
    type Strong = Toolbar;

    fn upgrade(&self) -> Option<Toolbar> {
        Some(Toolbar {
            container: self.container.upgrade()?,
            address: self.address.upgrade()?,
            spinner: self.spinner.upgrade()?,
        })
    }
}

impl Toolbar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, SPACING);
        container.add_css_class("toolbar");
        for button in &NAVIGATION_BUTTONS {
            container.append(&button.build());
        }
        let address = address_bar::build();
        let spinner = Spinner::new();
        container.append(&address);
        container.append(&spinner);
        Self {
            container,
            address,
            spinner,
        }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    pub fn show_address(&self, uri: &str) {
        self.address.set_text(uri);
    }

    pub fn focus_address(&self) {
        self.address.grab_focus();
        self.address.select_region(0, -1);
    }

    pub fn set_loading(&self, loading: bool) {
        self.spinner.set_spinning(loading);
    }

    pub fn show_bookmark_state(&self, state: BookmarkState) {
        address_bar::show_bookmark_state(&self.address, state);
    }

    pub fn add_end(&self, widget: &impl IsA<Widget>) {
        self.container.append(widget);
    }
}
