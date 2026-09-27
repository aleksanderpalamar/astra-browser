mod catalog;
mod handler;
mod library;
mod spec;

pub use catalog::BrowserAction;
pub use handler::{HistoryActions, install};
pub use library::LibraryAction;
pub use spec::{ActionSpec, register, register_accels};
