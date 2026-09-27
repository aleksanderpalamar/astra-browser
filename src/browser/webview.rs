use webkit6::prelude::*;
use webkit6::{NetworkError, WebView};

use crate::browser::media_support;

pub fn create() -> WebView {
    let webview = WebView::builder().hexpand(true).vexpand(true).build();
    report_failures(&webview);
    media_support::report_on_first_load(&webview);
    webview
}

fn report_failures(webview: &WebView) {
    webview.connect_load_failed(|_, _, uri, error| {
        if !error.matches(NetworkError::Cancelled) {
            eprintln!("Falha ao carregar {uri}: {error}");
        }
        false
    });
    webview.connect_web_process_terminated(|webview, reason| {
        let uri = webview.uri().unwrap_or_default();
        eprintln!("Processo web encerrado ({reason:?}) em {uri}");
    });
}
