use std::rc::Rc;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, Button, SearchEntry, glib};
use webkit6::prelude::*;
use webkit6::{LoadEvent, WebView};

use crate::browser::mode::BrowsingMode;
use crate::library::history_store::HistoryStore;
use crate::library::visit::Visit;
use crate::ui::actions::{self, LibraryAction};
use crate::ui::library::{LibraryPanel, PanelSpec, WeakLibraryPanel, link_row};
use crate::ui::tabs::Tabs;
use crate::ui::toolbar::Toolbar;

const PANEL: PanelSpec = PanelSpec {
    icon: "document-open-recent-symbolic",
    tooltip: "Histórico (Ctrl+H)",
    heading: "Histórico",
    empty_text: "Nenhuma página encontrada",
};
const VISIBLE_VISITS: usize = 100;

#[derive(Clone)]
struct HistoryView {
    panel: LibraryPanel,
    search: SearchEntry,
    store: Rc<HistoryStore>,
}

struct WeakHistoryView {
    panel: WeakLibraryPanel,
    search: glib::WeakRef<SearchEntry>,
    store: Rc<HistoryStore>,
}

impl glib::clone::Downgrade for HistoryView {
    type Weak = WeakHistoryView;

    fn downgrade(&self) -> WeakHistoryView {
        WeakHistoryView {
            panel: glib::clone::Downgrade::downgrade(&self.panel),
            search: ObjectExt::downgrade(&self.search),
            store: Rc::clone(&self.store),
        }
    }
}

impl glib::clone::Upgrade for WeakHistoryView {
    type Strong = HistoryView;

    fn upgrade(&self) -> Option<HistoryView> {
        Some(HistoryView {
            panel: self.panel.upgrade()?,
            search: self.search.upgrade()?,
            store: Rc::clone(&self.store),
        })
    }
}

impl HistoryView {
    fn refresh(&self) {
        let rows = self
            .store
            .search(&self.search.text(), VISIBLE_VISITS)
            .iter()
            .map(row)
            .collect();
        self.panel.show_rows(rows);
    }

    fn clear(&self) {
        self.store.clear();
        self.refresh();
    }

    fn connect_signals(&self, clear: &Button) {
        self.panel.connect_opening(glib::clone!(
            #[weak(rename_to = view)]
            self,
            move || {
                view.search.set_text("");
                view.refresh();
            }
        ));
        self.search.connect_search_changed(glib::clone!(
            #[weak(rename_to = view)]
            self,
            move |_| view.refresh()
        ));
        clear.connect_clicked(glib::clone!(
            #[weak(rename_to = view)]
            self,
            move |_| view.clear()
        ));
    }
}

pub fn install(
    window: &ApplicationWindow,
    tabs: &Tabs,
    toolbar: &Toolbar,
    store: Rc<HistoryStore>,
    mode: BrowsingMode,
) {
    if mode.records_history() {
        record_visits(tabs, &store);
    }
    let search = SearchEntry::builder()
        .placeholder_text("Buscar no histórico")
        .build();
    let clear = Button::builder().label("Limpar histórico").build();
    let panel = LibraryPanel::new(&PANEL, Some(search.upcast_ref()), Some(clear.upcast_ref()));
    toolbar.add_end(panel.button());
    let view = HistoryView {
        panel,
        search,
        store,
    };
    view.connect_signals(&clear);
    actions::register(
        window,
        LibraryAction::ShowHistory,
        glib::clone!(
            #[weak]
            view,
            move |_| view.panel.popup()
        ),
    );
}

fn record_visits(tabs: &Tabs, store: &Rc<HistoryStore>) {
    let store = Rc::clone(store);
    tabs.connect_tab_added(move |webview| {
        let on_load = Rc::clone(&store);
        webview.connect_load_changed(move |webview, event| {
            if event == LoadEvent::Finished {
                record_current_page(webview, &on_load);
            }
        });
        let on_title = Rc::clone(&store);
        webview.connect_title_notify(move |webview| {
            if webview.title().is_some_and(|title| !title.is_empty()) {
                record_current_page(webview, &on_title);
            }
        });
    });
}

fn record_current_page(webview: &WebView, store: &HistoryStore) {
    if let Some(uri) = webview.uri() {
        store.record(&uri, &webview.title().unwrap_or_default());
    }
}

fn row(visit: &Visit) -> GtkBox {
    let title = match visit.title.trim() {
        "" => visit.uri.as_str(),
        title => title,
    };
    link_row::build(title, &visit.uri)
}
