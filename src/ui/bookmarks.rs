use std::rc::Rc;

use gtk::{ApplicationWindow, Box as GtkBox, glib};
use webkit6::WebView;
use webkit6::prelude::*;

use crate::library::bookmark_store::BookmarkStore;
use crate::library::bookmarks::BookmarkState;
use crate::ui::actions::{self, LibraryAction};
use crate::ui::library::{LibraryPanel, PanelSpec, WeakLibraryPanel, link_row};
use crate::ui::tabs::{TabProperty, Tabs, WeakTabs};
use crate::ui::toolbar::{Toolbar, WeakToolbar};

const PANEL: PanelSpec = PanelSpec {
    icon: "user-bookmarks-symbolic",
    tooltip: "Favoritos (Ctrl+Shift+O)",
    heading: "Favoritos",
    empty_text: "Nenhum favorito ainda",
};

#[derive(Clone)]
struct Bookmarking {
    tabs: Tabs,
    toolbar: Toolbar,
    panel: LibraryPanel,
    store: Rc<BookmarkStore>,
}

struct WeakBookmarking {
    tabs: WeakTabs,
    toolbar: WeakToolbar,
    panel: WeakLibraryPanel,
    store: Rc<BookmarkStore>,
}

impl glib::clone::Downgrade for Bookmarking {
    type Weak = WeakBookmarking;

    fn downgrade(&self) -> WeakBookmarking {
        WeakBookmarking {
            tabs: glib::clone::Downgrade::downgrade(&self.tabs),
            toolbar: glib::clone::Downgrade::downgrade(&self.toolbar),
            panel: glib::clone::Downgrade::downgrade(&self.panel),
            store: Rc::clone(&self.store),
        }
    }
}

impl glib::clone::Upgrade for WeakBookmarking {
    type Strong = Bookmarking;

    fn upgrade(&self) -> Option<Bookmarking> {
        Some(Bookmarking {
            tabs: self.tabs.upgrade()?,
            toolbar: self.toolbar.upgrade()?,
            panel: self.panel.upgrade()?,
            store: Rc::clone(&self.store),
        })
    }
}

impl Bookmarking {
    fn toggle_current(&self) {
        let Some(webview) = self.tabs.current() else {
            return;
        };
        let Some(uri) = webview.uri() else {
            return;
        };
        let title = webview.title().unwrap_or_default();
        self.toolbar
            .show_bookmark_state(self.store.toggle(&uri, &title));
    }

    fn show_state_of(&self, webview: &WebView) {
        let state = webview.uri().map_or(BookmarkState::NotBookmarked, |uri| {
            self.store.state_of(&uri)
        });
        self.toolbar.show_bookmark_state(state);
    }

    fn refresh_list(&self) {
        let rows = self
            .store
            .entries()
            .iter()
            .map(|bookmark| self.row(&bookmark.title, &bookmark.uri))
            .collect();
        self.panel.show_rows(rows);
    }

    fn row(&self, title: &str, uri: &str) -> GtkBox {
        let row = link_row::build(title, uri);
        let uri = uri.to_owned();
        link_row::add_remove_button(
            &row,
            "Remover dos favoritos",
            glib::clone!(
                #[weak(rename_to = bookmarking)]
                self,
                move || bookmarking.remove(&uri)
            ),
        );
        row
    }

    fn remove(&self, uri: &str) {
        self.store.remove(uri);
        self.refresh_list();
        if let Some(webview) = self.tabs.current() {
            self.show_state_of(&webview);
        }
    }

    fn install_actions(&self, window: &ApplicationWindow) {
        actions::register(
            window,
            LibraryAction::ToggleBookmark,
            glib::clone!(
                #[weak(rename_to = bookmarking)]
                self,
                move |_| bookmarking.toggle_current()
            ),
        );
        actions::register(
            window,
            LibraryAction::ShowBookmarks,
            glib::clone!(
                #[weak(rename_to = bookmarking)]
                self,
                move |_| bookmarking.panel.popup()
            ),
        );
    }
}

pub fn install(
    window: &ApplicationWindow,
    tabs: &Tabs,
    toolbar: &Toolbar,
    store: Rc<BookmarkStore>,
) {
    let panel = LibraryPanel::new(&PANEL, None, None);
    toolbar.add_end(panel.button());
    let bookmarking = Bookmarking {
        tabs: tabs.clone(),
        toolbar: toolbar.clone(),
        panel,
        store,
    };
    bookmarking.install_actions(window);
    bookmarking.panel.connect_opening(glib::clone!(
        #[weak]
        bookmarking,
        move || bookmarking.refresh_list()
    ));
    tabs.watch_current(
        &[TabProperty::Uri],
        glib::clone!(
            #[weak]
            bookmarking,
            move |webview| bookmarking.show_state_of(webview)
        ),
    );
}
