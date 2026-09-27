mod app;
mod browser;
mod library;
mod ui;
mod utils;

use gtk::glib;

fn main() -> glib::ExitCode {
    app::run()
}
