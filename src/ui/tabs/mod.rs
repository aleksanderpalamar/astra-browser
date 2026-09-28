mod label;
mod order;
mod watch;
mod weak;

pub use order::Direction;
pub use watch::TabProperty;
pub use weak::WeakTabs;

use gtk::prelude::*;
use gtk::{Button, Notebook, PackType, glib};
use webkit6::prelude::*;
use webkit6::{NetworkSession, UserContentManager, WebView};

use crate::browser::webview;
use crate::ui::actions::{ActionSpec, BrowserAction};
use label::TabLabel;

#[derive(Clone)]
pub struct Tabs {
    notebook: Notebook,
    session: NetworkSession,
    content: UserContentManager,
}

impl Tabs {
    pub fn new(session: NetworkSession, content: UserContentManager) -> Self {
        let notebook = Notebook::builder()
            .scrollable(true)
            .show_border(false)
            .vexpand(true)
            .build();
        notebook.set_action_widget(&new_tab_button(), PackType::End);
        Self {
            notebook,
            session,
            content,
        }
    }

    pub fn widget(&self) -> &Notebook {
        &self.notebook
    }

    pub fn session(&self) -> &NetworkSession {
        &self.session
    }

    pub fn open(&self, uri: &str) {
        let webview = webview::create(&self.session, &self.content);
        let label = TabLabel::new(&webview);
        label.connect_close(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            #[weak]
            webview,
            move || tabs.close(&webview)
        ));
        self.open_popups_in_new_tabs(&webview);
        let page = self.notebook.append_page(&webview, Some(label.widget()));
        self.notebook.set_tab_reorderable(&webview, true);
        self.notebook.set_current_page(Some(page));
        webview.load_uri(uri);
    }

    pub fn current(&self) -> Option<WebView> {
        self.notebook
            .nth_page(self.notebook.current_page())
            .and_downcast()
    }

    pub fn is_current(&self, webview: &WebView) -> bool {
        let current = self.notebook.current_page();
        current.is_some() && self.notebook.page_num(webview) == current
    }

    pub fn close(&self, webview: &WebView) {
        let Some(page) = self.notebook.page_num(webview) else {
            return;
        };
        self.notebook.remove_page(Some(page));
        if self.notebook.n_pages() > 0 {
            return;
        }
        if let Err(error) = self.notebook.activate_action("window.close", None) {
            eprintln!("Não foi possível fechar a janela: {error}");
        }
    }

    pub fn close_current(&self) {
        if let Some(webview) = self.current() {
            self.close(&webview);
        }
    }

    pub fn close_all(&self) {
        while self.notebook.n_pages() > 0 {
            self.notebook.remove_page(None);
        }
    }

    pub fn select(&self, direction: Direction) {
        let Some(current) = self.notebook.current_page() else {
            return;
        };
        let Some(target) = order::neighbor(current, self.notebook.n_pages(), direction) else {
            return;
        };
        self.notebook.set_current_page(Some(target));
    }

    pub fn connect_tab_added<F: Fn(&WebView) + 'static>(&self, callback: F) {
        self.notebook.connect_page_added(move |_, child, _| {
            if let Some(webview) = child.downcast_ref::<WebView>() {
                callback(webview);
            }
        });
    }

    pub fn connect_tab_selected<F: Fn(&WebView) + 'static>(&self, callback: F) {
        self.notebook.connect_switch_page(move |_, child, _| {
            if let Some(webview) = child.downcast_ref::<WebView>() {
                callback(webview);
            }
        });
    }

    fn open_popups_in_new_tabs(&self, webview: &WebView) {
        webview.connect_create(glib::clone!(
            #[weak(rename_to = tabs)]
            self,
            #[upgrade_or_default]
            move |_, action| {
                if !action.is_user_gesture() {
                    return None;
                }
                if let Some(uri) = action.request().and_then(|request| request.uri()) {
                    tabs.open(&uri);
                }
                None
            }
        ));
    }
}

fn new_tab_button() -> Button {
    let button = Button::builder()
        .icon_name("tab-new-symbolic")
        .tooltip_text("Nova aba (Ctrl+T)")
        .action_name(BrowserAction::NewTab.detailed_name())
        .build();
    button.add_css_class("flat");
    button
}
