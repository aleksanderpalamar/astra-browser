mod download;
mod filters;
mod sanitize;

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use gtk::{gio, glib};
use webkit6::{UserContentFilter, UserContentFilterStore, UserContentManager};

use crate::library::files;
use crate::utils::clock::unix_now;

#[derive(Clone)]
pub struct AdBlocker {
    content: UserContentManager,
    store: UserContentFilterStore,
    timestamp: PathBuf,
    enabled: Rc<Cell<bool>>,
}

impl AdBlocker {
    pub fn new(directory: &Path) -> Self {
        Self {
            content: UserContentManager::new(),
            store: UserContentFilterStore::new(&directory.to_string_lossy()),
            timestamp: directory.join(filters::TIMESTAMP_FILE),
            enabled: Rc::new(Cell::new(false)),
        }
    }

    pub fn content_manager(&self) -> &UserContentManager {
        &self.content
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
        if enabled {
            glib::spawn_future_local(self.clone().activate());
        } else {
            self.content.remove_all_filters();
        }
    }

    async fn activate(self) {
        match self.store.load_future(filters::FILTER_ID).await {
            Ok(filter) => {
                self.install(&filter);
                if self.needs_update() {
                    self.update().await;
                }
            }
            Err(_) => self.update().await,
        }
    }

    async fn update(&self) {
        let source = match download::fetch(filters::FILTER_URL).await {
            Ok(source) => source,
            Err(error) => {
                eprintln!("Não foi possível baixar a lista de bloqueio: {error}");
                return;
            }
        };
        let Some(source) = repaired(source).await else {
            return;
        };
        match self.store.save_future(filters::FILTER_ID, &source).await {
            Ok(filter) => {
                self.install(&filter);
                self.record_update();
            }
            Err(error) => eprintln!("Não foi possível compilar a lista de bloqueio: {error}"),
        }
    }

    fn install(&self, filter: &UserContentFilter) {
        if !self.enabled.get() {
            return;
        }
        self.content.remove_all_filters();
        self.content.add_filter(filter);
    }

    fn needs_update(&self) -> bool {
        let last_update = files::read_or_empty(&self.timestamp)
            .ok()
            .and_then(|contents| filters::parse_timestamp(&contents));
        filters::needs_update(last_update, unix_now())
    }

    fn record_update(&self) {
        if let Err(error) = files::write_atomically(&self.timestamp, &unix_now().to_string()) {
            eprintln!("Não foi possível registrar a atualização da lista de bloqueio: {error}");
        }
    }
}

async fn repaired(source: glib::Bytes) -> Option<glib::Bytes> {
    match gio::spawn_blocking(move || sanitize::sanitize(&source)).await {
        Ok(Ok(rules)) => Some(glib::Bytes::from_owned(rules)),
        Ok(Err(error)) => {
            eprintln!("Lista de bloqueio inválida: {error}");
            None
        }
        Err(_) => {
            eprintln!("A correção da lista de bloqueio foi interrompida");
            None
        }
    }
}
