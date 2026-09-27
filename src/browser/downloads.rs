use std::path::{Path, PathBuf};

use gtk::glib;
use gtk::prelude::*;
use webkit6::prelude::*;
use webkit6::{Download, PolicyDecisionType, ResponsePolicyDecision, WebView};

use crate::browser::download_name::unique_destination;

pub fn download_unsupported_responses(webview: &WebView) {
    webview.connect_decide_policy(|_, decision, kind| {
        if kind != PolicyDecisionType::Response {
            return false;
        }
        let Some(response) = decision.downcast_ref::<ResponsePolicyDecision>() else {
            return false;
        };
        if response.is_mime_type_supported() || !response.is_main_frame_main_resource() {
            return false;
        }
        decision.download();
        true
    });
}

pub fn save_in_downloads_folder(download: &Download) {
    download.connect_decide_destination(|download, suggested| {
        let destination = unique_destination(&downloads_directory(), suggested, Path::exists);
        let Some(destination) = destination.to_str() else {
            eprintln!("Destino de download inválido: {}", destination.display());
            return false;
        };
        download.set_destination(destination);
        true
    });
}

fn downloads_directory() -> PathBuf {
    glib::user_special_dir(glib::UserDirectory::Downloads).unwrap_or_else(glib::home_dir)
}
