pub mod bookmark_store;
pub mod bookmarks;
pub mod files;
pub mod history;
pub mod history_store;
pub mod tsv;
pub mod visit;

use std::rc::Rc;

use bookmark_store::BookmarkStore;
use history_store::HistoryStore;

const BOOKMARKS_FILE: &str = "bookmarks.tsv";
const HISTORY_FILE: &str = "history.tsv";

#[derive(Clone)]
pub struct Library {
    pub bookmarks: Rc<BookmarkStore>,
    pub history: Rc<HistoryStore>,
}

impl Library {
    pub fn open() -> Self {
        Self {
            bookmarks: Rc::new(BookmarkStore::open(files::data_path(BOOKMARKS_FILE))),
            history: Rc::new(HistoryStore::open(files::data_path(HISTORY_FILE))),
        }
    }
}
