mod row;
mod status;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Root, glib};
use webkit6::{Download, NetworkSession};

use crate::browser::downloads;
use crate::ui::actions::{self, LibraryAction};
use crate::ui::library::{LibraryPanel, PanelSpec};
use crate::ui::toolbar::Toolbar;

const PANEL: PanelSpec = PanelSpec {
    icon: "folder-download-symbolic",
    tooltip: "Downloads (Ctrl+Shift+Y)",
    heading: "Downloads",
    empty_text: "Nenhum download nesta sessão",
};

pub fn install(window: &ApplicationWindow, toolbar: &Toolbar, session: &NetworkSession) {
    let panel = LibraryPanel::new(&PANEL, None, None);
    toolbar.add_end(panel.button());
    actions::register(
        window,
        LibraryAction::ShowDownloads,
        glib::clone!(
            #[weak]
            panel,
            move |_| panel.popup()
        ),
    );
    session.connect_download_started(glib::clone!(
        #[weak]
        window,
        #[weak]
        panel,
        move |_, download| {
            if !belongs_to(download, &window) {
                return;
            }
            downloads::save_in_downloads_folder(download);
            panel.prepend_row(&row::build(download));
            panel.popup();
        }
    ));
}

fn belongs_to(download: &Download, window: &ApplicationWindow) -> bool {
    let window: &Root = window.upcast_ref();
    download
        .web_view()
        .and_then(|webview| webview.root())
        .is_some_and(|root| &root == window)
}
