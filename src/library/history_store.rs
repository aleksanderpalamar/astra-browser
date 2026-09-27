use std::cell::RefCell;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::library::files;
use crate::library::history::{History, Recording};
use crate::library::visit::Visit;

pub struct HistoryStore {
    path: PathBuf,
    history: RefCell<History>,
}

impl HistoryStore {
    pub fn open(path: PathBuf) -> Self {
        let contents = files::read_or_empty(&path).unwrap_or_else(|error| {
            eprintln!(
                "Não foi possível ler o histórico em {}: {error}",
                path.display()
            );
            String::new()
        });
        let history = History::parse(&contents);
        let store = Self {
            path,
            history: RefCell::new(history),
        };
        if contents.lines().count() > store.history.borrow().len() {
            store.rewrite();
        }
        store
    }

    pub fn record(&self, uri: &str, title: &str) {
        let visit = Visit {
            visited_at: now(),
            uri: uri.to_owned(),
            title: title.to_owned(),
        };
        if self.history.borrow_mut().record(visit) == Recording::Ignored {
            return;
        }
        let Some(line) = self.history.borrow().last().map(Visit::encode) else {
            return;
        };
        if let Err(error) = files::append_line(&self.path, &line) {
            eprintln!(
                "Não foi possível salvar o histórico em {}: {error}",
                self.path.display()
            );
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<Visit> {
        self.history.borrow().search(query, limit)
    }

    pub fn clear(&self) {
        self.history.borrow_mut().clear();
        self.rewrite();
    }

    fn rewrite(&self) {
        let contents = self.history.borrow().serialize();
        if let Err(error) = files::write_atomically(&self.path, &contents) {
            eprintln!(
                "Não foi possível gravar o histórico em {}: {error}",
                self.path.display()
            );
        }
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}
