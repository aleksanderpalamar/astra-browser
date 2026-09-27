use gtk::prelude::*;
use gtk::{ApplicationWindow, gio, glib};
use webkit6::WebView;
use webkit6::prelude::*;

use super::BrowserAction;
use crate::browser::navigation::{self, HOME_URI};
use crate::ui::tabs::{Direction, Tabs};
use crate::ui::toolbar::Toolbar;

#[derive(Clone)]
pub struct HistoryActions {
    back: gio::SimpleAction,
    forward: gio::SimpleAction,
}

impl HistoryActions {
    pub fn update(&self, webview: &WebView) {
        self.back.set_enabled(webview.can_go_back());
        self.forward.set_enabled(webview.can_go_forward());
    }
}

#[derive(Clone)]
struct Handler {
    tabs: Tabs,
    toolbar: Toolbar,
}

impl Handler {
    fn perform(&self, action: BrowserAction, parameter: Option<&glib::Variant>) {
        match action {
            BrowserAction::Back => self.with_current(|webview| webview.go_back()),
            BrowserAction::Forward => self.with_current(|webview| webview.go_forward()),
            BrowserAction::Reload => self.with_current(|webview| webview.reload()),
            BrowserAction::Home => self.with_current(navigation::go_home),
            BrowserAction::FocusAddress => self.toolbar.focus_address(),
            BrowserAction::OpenAddress => self.open_address(parameter),
            BrowserAction::NewTab => self.new_tab(),
            BrowserAction::CloseTab => self.tabs.close_current(),
            BrowserAction::NextTab => self.tabs.select(Direction::Next),
            BrowserAction::PreviousTab => self.tabs.select(Direction::Previous),
        }
    }

    fn with_current(&self, operation: impl FnOnce(&WebView)) {
        if let Some(webview) = self.tabs.current() {
            operation(&webview);
        }
    }

    fn open_address(&self, parameter: Option<&glib::Variant>) {
        if let Some(input) = parameter.and_then(glib::Variant::str) {
            self.with_current(|webview| navigation::open(webview, input));
        }
    }

    fn new_tab(&self) {
        self.tabs.open(HOME_URI);
        self.toolbar.focus_address();
    }
}

pub fn install(window: &ApplicationWindow, tabs: Tabs, toolbar: Toolbar) -> HistoryActions {
    let handler = Handler { tabs, toolbar };
    let history = HistoryActions {
        back: register(window, BrowserAction::Back, &handler),
        forward: register(window, BrowserAction::Forward, &handler),
    };
    BrowserAction::ALL
        .into_iter()
        .filter(|action| !action.depends_on_history())
        .for_each(|action| {
            register(window, action, &handler);
        });
    history
}

fn register(
    window: &ApplicationWindow,
    action: BrowserAction,
    handler: &Handler,
) -> gio::SimpleAction {
    let simple = gio::SimpleAction::new(action.name(), action.parameter_type());
    let handler = handler.clone();
    simple.connect_activate(move |_, parameter| handler.perform(action, parameter));
    window.add_action(&simple);
    simple
}
