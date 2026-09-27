mod question;

use gtk::prelude::*;
use gtk::{AlertDialog, Window, gio};
use webkit6::prelude::*;
use webkit6::{PermissionRequest, UserMediaPermissionRequest, WebView};

use crate::ui::tabs::Tabs;
use question::{CaptureDevices, permission_question};

const BLOCK_BUTTON: i32 = 0;
const ALLOW_BUTTON: i32 = 1;
const NO_DEFAULT_BUTTON: i32 = -1;
const DETAIL: &str =
    "A permissão vale para este pedido; recarregar a página faz o site perguntar de novo.";

pub fn install(tabs: &Tabs) {
    tabs.connect_tab_added(|webview| {
        webview.connect_permission_request(ask_for_capture_devices);
    });
}

fn ask_for_capture_devices(webview: &WebView, request: &PermissionRequest) -> bool {
    let Some(media) = request.downcast_ref::<UserMediaPermissionRequest>() else {
        return false;
    };
    let Some(devices) =
        CaptureDevices::requested(media.is_for_audio_device(), media.is_for_video_device())
    else {
        return false;
    };
    let question = permission_question(webview.uri().as_deref(), devices);
    ask(webview, &question, request.clone());
    true
}

fn ask(webview: &WebView, question: &str, request: PermissionRequest) {
    let dialog = AlertDialog::builder()
        .message(question)
        .detail(DETAIL)
        .buttons(["Bloquear", "Permitir"])
        .cancel_button(BLOCK_BUTTON)
        .default_button(NO_DEFAULT_BUTTON)
        .modal(true)
        .build();
    let parent = webview.root().and_downcast::<Window>();
    dialog.choose(
        parent.as_ref(),
        gio::Cancellable::NONE,
        move |choice| match choice {
            Ok(ALLOW_BUTTON) => request.allow(),
            _ => request.deny(),
        },
    );
}
