use gtk::glib;
use webkit6::prelude::*;
use webkit6::{NetworkError, NetworkSession, PolicyError, UserContentManager, WebView};

use crate::browser::{crash, downloads, media_support};

pub fn create(session: &NetworkSession, content: &UserContentManager) -> WebView {
    let webview = WebView::builder()
        .network_session(session)
        .user_content_manager(content)
        .hexpand(true)
        .vexpand(true)
        .build();
    configure(&webview);
    report_failures(&webview);
    crash::recover_on_termination(&webview);
    downloads::download_unsupported_responses(&webview);
    media_support::report_on_first_load(&webview);
    webview
}

fn configure(webview: &WebView) {
    let Some(settings) = WebViewExt::settings(webview) else {
        eprintln!("Configurações do WebView indisponíveis");
        return;
    };
    settings.set_enable_developer_extras(true);
    settings.set_enable_media_stream(true);
    settings.set_enable_webrtc(true);
    settings.set_enable_smooth_scrolling(false);
}

fn report_failures(webview: &WebView) {
    webview.connect_load_failed(|_, _, uri, error| {
        if !is_expected_interruption(error) {
            eprintln!("Falha ao carregar {uri}: {error}");
        }
        false
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
