mod catalog;
mod handler;
mod library;
mod spec;

pub use catalog::BrowserAction;
pub use handler::{BackForwardActions, install};
pub use library::LibraryAction;
pub use spec::{ActionSpec, register, register_accels};
