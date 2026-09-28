mod download;
mod filters;
mod sanitize;
mod youtube;

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use gtk::{gio, glib};
use webkit6::{UserContentFilter, UserContentFilterStore, UserContentManager, UserScript};

use crate::library::files;
use crate::utils::clock::unix_now;

#[derive(Clone)]
pub struct AdBlocker {
    content: UserContentManager,
    store: UserContentFilterStore,
    timestamp: PathBuf,
    extra_source: PathBuf,
    player_ads: UserScript,
    enabled: Rc<Cell<bool>>,
}

impl AdBlocker {
    pub fn new(directory: &Path) -> Self {
        Self {
            content: UserContentManager::new(),
            store: UserContentFilterStore::new(&directory.to_string_lossy()),
            timestamp: directory.join(filters::TIMESTAMP_FILE),
            extra_source: directory.join(filters::EXTRA_RULES_SOURCE_FILE),
            player_ads: youtube::player_ads_script(),
            enabled: Rc::new(Cell::new(false)),
        }
    }

    pub fn content_manager(&self) -> &UserContentManager {
        &self.content
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
        self.content.remove_script(&self.player_ads);
        if enabled {
            self.content.add_script(&self.player_ads);
            glib::spawn_future_local(self.clone().activate());
        } else {
            self.content.remove_all_filters();
        }
    }

    async fn activate(self) {
        self.install_extra_rules().await;
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

    async fn install_extra_rules(&self) {
        match self.compiled_extra_rules().await {
            Some(filter) => self.install(&filter),
            None => self.compile_extra_rules().await,
        }
    }

    async fn compiled_extra_rules(&self) -> Option<UserContentFilter> {
        let compiled_source = files::read_or_empty(&self.extra_source).ok()?;
        if !filters::is_current_extra_rules(&compiled_source) {
            return None;
        }
        self.store.load_future(filters::EXTRA_FILTER_ID).await.ok()
    }

    async fn compile_extra_rules(&self) {
        let rules = glib::Bytes::from_static(filters::EXTRA_RULES.as_bytes());
        match self
            .store
            .save_future(filters::EXTRA_FILTER_ID, &rules)
            .await
        {
            Ok(filter) => {
                self.install(&filter);
                self.record_extra_rules();
            }
            Err(error) => {
                eprintln!("Não foi possível compilar as regras extras de bloqueio: {error}")
            }
        }
    }

    fn record_extra_rules(&self) {
        if let Err(error) = files::write_atomically(&self.extra_source, filters::EXTRA_RULES) {
            eprintln!("Não foi possível registrar as regras extras compiladas: {error}");
        }
    }

    fn install(&self, filter: &UserContentFilter) {
        if self.enabled.get() {
            self.content.add_filter(filter);
        }
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
