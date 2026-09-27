use webkit6::WebView;
use webkit6::prelude::*;

use crate::utils::url::resolve_address;

pub const HOME_URI: &str = "https://duckduckgo.com";

#[derive(Clone)]
pub struct Navigator {
    webview: WebView,
}

impl Navigator {
    pub fn new(webview: WebView) -> Self {
        Self { webview }
    }

    pub fn open(&self, input: &str) {
        let Some(uri) = resolve_address(input) else {
            return;
        };
        self.webview.load_uri(&uri);
    }

    pub fn home(&self) {
        self.webview.load_uri(HOME_URI);
    }

    pub fn back(&self) {
        self.webview.go_back();
    }

    pub fn forward(&self) {
        self.webview.go_forward();
    }

    pub fn reload(&self) {
        self.webview.reload();
    }
}
