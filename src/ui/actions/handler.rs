use gtk::{ApplicationWindow, gio, glib};
use webkit6::WebView;
use webkit6::prelude::*;

use super::{BrowserAction, register};
use crate::browser::inspector;
use crate::browser::navigation::{self, HOME_URI};
use crate::ui::tabs::{Direction, Tabs};
use crate::ui::toolbar::Toolbar;

#[derive(Clone)]
pub struct BackForwardActions {
    back: gio::SimpleAction,
    forward: gio::SimpleAction,
}

impl BackForwardActions {
    pub fn update(&self, webview: &WebView) {
        self.back.set_enabled(webview.can_go_back());
        self.forward.set_enabled(webview.can_go_forward());
    }
}

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
            BrowserAction::ToggleInspector => self.with_current(inspector::toggle),
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

pub fn install(window: &ApplicationWindow, tabs: &Tabs, toolbar: &Toolbar) -> BackForwardActions {
    let back_forward = BackForwardActions {
        back: register_browser_action(window, BrowserAction::Back, tabs, toolbar),
        forward: register_browser_action(window, BrowserAction::Forward, tabs, toolbar),
    };
    BrowserAction::ALL
        .into_iter()
        .filter(|action| !action.depends_on_history())
        .for_each(|action| {
            register_browser_action(window, action, tabs, toolbar);
        });
    back_forward
}

fn register_browser_action(
    window: &ApplicationWindow,
    action: BrowserAction,
    tabs: &Tabs,
    toolbar: &Toolbar,
) -> gio::SimpleAction {
    register(
        window,
        action,
        glib::clone!(
            #[weak]
            tabs,
            #[weak]
            toolbar,
            move |parameter| Handler { tabs, toolbar }.perform(action, parameter)
        ),
    )
}
