use webkit6::WebView;
use webkit6::prelude::*;

pub fn enable(webview: &WebView) {
    match WebViewExt::settings(webview) {
        Some(settings) => settings.set_enable_developer_extras(true),
        None => eprintln!("Configurações do WebView indisponíveis; inspetor desativado"),
    }
}

pub fn toggle(webview: &WebView) {
    let Some(inspector) = webview.inspector() else {
        eprintln!("Inspetor indisponível para esta página");
        return;
    };
    if inspector.web_view().is_some() {
        inspector.close();
    } else {
        inspector.show();
    }
}
