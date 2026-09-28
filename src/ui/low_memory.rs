use std::rc::Rc;

use gtk::{Application, glib};

use crate::browser::memory_mode::MemoryMode;
use crate::browser::webview::WebViewFactory;
use crate::library::preferences::Preferences;
use crate::ui::actions::{self, AppAction};

pub fn install(app: &Application, webviews: &WebViewFactory, preferences: Rc<Preferences>) {
    let enabled = preferences.low_memory_enabled();
    webviews.set_memory_mode(MemoryMode::from_low_memory(enabled));
    actions::register_toggle(
        app,
        AppAction::ToggleLowMemory,
        enabled,
        glib::clone!(
            #[strong]
            webviews,
            move |enabled| {
                webviews.set_memory_mode(MemoryMode::from_low_memory(enabled));
                preferences.set_low_memory_enabled(enabled);
            }
        ),
    );
}
