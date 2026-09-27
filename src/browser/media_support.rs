use std::cell::Cell;

use gtk::glib;
use webkit6::prelude::*;
use webkit6::{LoadEvent, WebView};

use crate::browser::media_formats::{ESSENTIAL_FORMATS, unsupported_formats_warning};

const ISOLATED_WORLD: &str = "rust-browser-media-support";
const PROBE_SCRIPT: &str =
    "return mimeTypes.split('\\n').filter(type => !MediaSource.isTypeSupported(type)).join('\\n');";

pub fn report_on_first_load(webview: &WebView) {
    let reported = Cell::new(false);
    webview.connect_load_changed(move |webview, event| {
        if event != LoadEvent::Finished || reported.replace(true) {
            return;
        }
        glib::spawn_future_local(report_unsupported_formats(webview.clone()));
    });
}

async fn report_unsupported_formats(webview: WebView) {
    let probe = webview
        .call_async_javascript_function_future(
            PROBE_SCRIPT,
            Some(&probe_arguments()),
            Some(ISOLATED_WORLD),
            None,
        )
        .await;
    match probe {
        Ok(unsupported) => {
            if let Some(warning) = unsupported_formats_warning(&unsupported.to_str()) {
                eprintln!("{warning}");
            }
        }
        Err(error) => eprintln!("Não foi possível verificar o suporte a mídia: {error}"),
    }
}

fn probe_arguments() -> glib::Variant {
    let mime_types: Vec<&str> = ESSENTIAL_FORMATS
        .iter()
        .map(|format| format.mime_type)
        .collect();
    let arguments = glib::VariantDict::new(None);
    arguments.insert("mimeTypes", mime_types.join("\n"));
    arguments.end()
}
