mod page;
mod policy;

use std::cell::RefCell;
use std::time::Instant;

use webkit6::prelude::*;
use webkit6::{WebProcessTerminationReason, WebView};

use policy::{CrashRecovery, Recovery};

pub fn recover_on_termination(webview: &WebView) {
    let recovery = RefCell::new(CrashRecovery::default());
    webview.connect_web_process_terminated(move |webview, reason| {
        let uri = webview.uri().unwrap_or_default();
        eprintln!("Processo web encerrado ({reason:?}) em {uri}");
        if uri.is_empty() {
            return;
        }
        let decision = recovery.borrow_mut().decide(reason, &uri, Instant::now());
        match decision {
            Recovery::Reload => webview.reload(),
            Recovery::ShowFailure => show_failure(webview, reason, &uri),
            Recovery::GiveUp => eprintln!("A página de falha também travou em {uri}"),
        }
    });
}

fn show_failure(webview: &WebView, reason: WebProcessTerminationReason, uri: &str) {
    webview.load_alternate_html(&page::failure_page(reason, uri), uri, None);
}
