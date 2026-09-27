use webkit6::WebView;
use webkit6::prelude::*;

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
