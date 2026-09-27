use std::cell::RefCell;
use std::path::PathBuf;

use crate::library::bookmarks::{Bookmark, BookmarkState, Bookmarks};
use crate::library::files;

pub struct BookmarkStore {
    path: PathBuf,
    bookmarks: RefCell<Bookmarks>,
}

impl BookmarkStore {
    pub fn open(path: PathBuf) -> Self {
        let bookmarks = match files::read_or_empty(&path) {
            Ok(contents) => Bookmarks::parse(&contents),
            Err(error) => {
                eprintln!(
                    "Não foi possível ler os favoritos em {}: {error}",
                    path.display()
                );
                Bookmarks::default()
            }
        };
        Self {
            path,
            bookmarks: RefCell::new(bookmarks),
        }
    }

    pub fn state_of(&self, uri: &str) -> BookmarkState {
        self.bookmarks.borrow().state_of(uri)
    }

    pub fn entries(&self) -> Vec<Bookmark> {
        self.bookmarks.borrow().entries().to_vec()
    }

    pub fn toggle(&self, uri: &str, title: &str) -> BookmarkState {
        let state = self.bookmarks.borrow_mut().toggle(uri, title);
        self.save();
        state
    }

    pub fn remove(&self, uri: &str) {
        self.bookmarks.borrow_mut().remove(uri);
        self.save();
    }

    fn save(&self) {
        let contents = self.bookmarks.borrow().serialize();
        if let Err(error) = files::write_atomically(&self.path, &contents) {
            eprintln!(
                "Não foi possível salvar os favoritos em {}: {error}",
                self.path.display()
            );
        }
    }
}
