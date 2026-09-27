use webkit6::WebView;
use webkit6::prelude::*;

use crate::utils::url::resolve_address;

pub const HOME_URI: &str = "https://duckduckgo.com";

pub fn open(webview: &WebView, input: &str) {
    let Some(uri) = resolve_address(input) else {
        return;
    };
    webview.load_uri(&uri);
}

pub fn go_home(webview: &WebView) {
    webview.load_uri(HOME_URI);
}
