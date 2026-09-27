pub mod bookmark_store;
pub mod bookmarks;
pub mod files;
pub mod history;
pub mod history_store;
pub mod preferences;
pub mod tsv;
pub mod visit;

use std::rc::Rc;

use bookmark_store::BookmarkStore;
use history_store::HistoryStore;
use preferences::Preferences;

const BOOKMARKS_FILE: &str = "bookmarks.tsv";
const HISTORY_FILE: &str = "history.tsv";
const PREFERENCES_FILE: &str = "preferences.ini";

#[derive(Clone)]
pub struct Library {
    pub bookmarks: Rc<BookmarkStore>,
    pub history: Rc<HistoryStore>,
    pub preferences: Rc<Preferences>,
}

impl Library {
    pub fn open() -> Self {
        Self {
            bookmarks: Rc::new(BookmarkStore::open(files::data_path(BOOKMARKS_FILE))),
            history: Rc::new(HistoryStore::open(files::data_path(HISTORY_FILE))),
            preferences: Rc::new(Preferences::open(files::config_path(PREFERENCES_FILE))),
        }
    }
}
