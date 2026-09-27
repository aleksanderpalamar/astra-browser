use gtk::{MenuButton, gio};

use crate::ui::actions::{ActionSpec, AppAction, BrowserAction};

pub fn button() -> MenuButton {
    let windows = gio::Menu::new();
    append(&windows, "Nova aba", BrowserAction::NewTab);
    append(&windows, "Nova janela", AppAction::NewWindow);
    append(&windows, "Nova janela privada", AppAction::NewPrivateWindow);
    let application = gio::Menu::new();
    append(&application, "Sair", AppAction::Quit);
    let menu = gio::Menu::new();
    menu.append_section(None, &windows);
    menu.append_section(None, &application);
    MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Menu principal")
        .menu_model(&menu)
        .primary(true)
        .build()
}

fn append(menu: &gio::Menu, label: &str, action: impl ActionSpec) {
    menu.append(Some(label), Some(&action.detailed_name()));
}
