use gtk::glib;
use webkit6::prelude::*;
use webkit6::{NetworkError, NetworkSession, PolicyError, WebView};

use crate::browser::{downloads, inspector, media_support};

pub fn create(session: &NetworkSession) -> WebView {
    let webview = WebView::builder()
        .network_session(session)
        .hexpand(true)
        .vexpand(true)
        .build();
    report_failures(&webview);
    inspector::enable(&webview);
    downloads::download_unsupported_responses(&webview);
    media_support::report_on_first_load(&webview);
    webview
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
