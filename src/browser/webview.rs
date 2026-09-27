use webkit6::prelude::*;
use webkit6::{NetworkError, WebView};

pub fn create() -> WebView {
    let webview = WebView::builder().hexpand(true).vexpand(true).build();
    report_failures(&webview);
    open_new_windows_in_place(&webview);
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

fn open_new_windows_in_place(webview: &WebView) {
    webview.connect_create(|webview, action| {
        if !action.is_user_gesture() {
            return None;
        }
        if let Some(uri) = action.request().and_then(|request| request.uri()) {
            webview.load_uri(&uri);
        }
        None
    });
}
