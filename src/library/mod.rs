pub mod bookmark_store;
pub mod bookmarks;
pub mod files;
pub mod tsv;

use std::rc::Rc;

use bookmark_store::BookmarkStore;

const BOOKMARKS_FILE: &str = "bookmarks.tsv";

#[derive(Clone)]
pub struct Library {
    pub bookmarks: Rc<BookmarkStore>,
}

impl Library {
    pub fn open() -> Self {
        Self {
            bookmarks: Rc::new(BookmarkStore::open(files::data_path(BOOKMARKS_FILE))),
        }
    }
}
