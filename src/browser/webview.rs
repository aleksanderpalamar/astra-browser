use gtk::glib;
use webkit6::prelude::*;
use webkit6::{
    NetworkError, NetworkSession, PolicyError, Settings, UserContentManager, WebContext, WebView,
};

use crate::browser::memory_mode::MemoryMode;
use crate::browser::{downloads, media_support};

#[derive(Clone)]
pub struct WebViewFactory {
    context: WebContext,
    content: UserContentManager,
    settings: Settings,
}

impl WebViewFactory {
    pub fn new(context: WebContext, content: UserContentManager) -> Self {
        Self {
            context,
            content,
            settings: shared_settings(),
        }
    }

    pub fn set_memory_mode(&self, mode: MemoryMode) {
        self.context.set_cache_model(mode.cache_model());
        self.settings.set_enable_page_cache(mode.keeps_page_cache());
    }

    pub fn create(&self, session: &NetworkSession) -> WebView {
        let webview = WebView::builder()
            .web_context(&self.context)
            .network_session(session)
            .user_content_manager(&self.content)
            .settings(&self.settings)
            .hexpand(true)
            .vexpand(true)
            .build();
        report_failures(&webview);
        downloads::download_unsupported_responses(&webview);
        media_support::report_on_first_load(&webview);
        webview
    }
}

fn shared_settings() -> Settings {
    let settings = Settings::new();
    settings.set_enable_developer_extras(true);
    settings.set_enable_media_stream(true);
    settings.set_enable_webrtc(true);
    settings.set_enable_smooth_scrolling(false);
    settings
}

fn report_failures(webview: &WebView) {
    webview.connect_load_failed(|_, _, uri, error| {
        if !is_expected_interruption(error) {
            eprintln!("Falha ao carregar {uri}: {error}");
        }
        false
    });
    webview.connect_web_process_terminated(|webview, reason| {
        let uri = webview.uri().unwrap_or_default();
        eprintln!("Processo web encerrado ({reason:?}) em {uri}");
    });
}

fn is_expected_interruption(error: &glib::Error) -> bool {
    error.matches(NetworkError::Cancelled)
        || error.matches(PolicyError::FrameLoadInterruptedByPolicyChange)
}

#[cfg(test)]
mod tests {
    use gtk::glib;
    use webkit6::{NetworkError, PolicyError};

    use super::is_expected_interruption;

    #[test]
    fn cancelled_loads_and_downloads_are_expected() {
        let cancelled = glib::Error::new(NetworkError::Cancelled, "cancelado");
        let download =
            glib::Error::new(PolicyError::FrameLoadInterruptedByPolicyChange, "download");
        assert!(is_expected_interruption(&cancelled));
        assert!(is_expected_interruption(&download));
    }

    #[test]
    fn real_failures_are_reported() {
        let failure = glib::Error::new(NetworkError::Failed, "sem rede");
        let mime = glib::Error::new(PolicyError::CannotShowMimeType, "mime");
        assert!(!is_expected_interruption(&failure));
        assert!(!is_expected_interruption(&mime));
    }
}
