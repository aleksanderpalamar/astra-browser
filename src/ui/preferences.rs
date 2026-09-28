use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Frame, Label, ListBox,
    Orientation, SelectionMode, Switch, Widget, Window, pango,
};

use crate::ui::actions::{ActionSpec, AppAction, BrowserAction};

const WINDOW_NAME: &str = "astra-preferences";
const GPU_DIAGNOSTICS_URI: &str = "webkit://gpu";
const MARGIN: i32 = 24;
const ROW_MARGIN: i32 = 12;
const SPACING: i32 = 12;

struct Setting {
    title: &'static str,
    description: &'static str,
    action: AppAction,
}

const AD_BLOCK: Setting = Setting {
    title: "Bloquear anúncios",
    description: "Bloqueia redes de anúncio e esconde anúncios nas páginas, inclusive no YouTube. Recarregue a página para ver o efeito.",
    action: AppAction::ToggleAdBlock,
};

const HARDWARE_ACCELERATION: Setting = Setting {
    title: "Usar aceleração de hardware",
    description: "Usa a GPU para desenhar as páginas. Desative se aparecerem falhas gráficas.",
    action: AppAction::ToggleHardwareAcceleration,
};

pub fn show(app: &Application) {
    if let Some(window) = existing(app) {
        window.present();
        return;
    }
    let window = Window::builder()
        .application(app)
        .title("Configurações")
        .default_width(560)
        .default_height(420)
        .child(&page())
        .build();
    window.set_widget_name(WINDOW_NAME);
    if let Some(browser) = browser_window(app) {
        window.set_transient_for(Some(&browser));
    }
    window.present();
}

fn existing(app: &Application) -> Option<Window> {
    app.windows()
        .into_iter()
        .find(|window| window.widget_name() == WINDOW_NAME)
}

fn browser_window(app: &Application) -> Option<Window> {
    app.windows()
        .into_iter()
        .find(|window| window.is::<ApplicationWindow>())
}

fn page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(SPACING)
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .margin_start(MARGIN)
        .margin_end(MARGIN)
        .build();
    page.append(&heading("Privacidade e segurança"));
    page.append(&section(&[switch_row(&AD_BLOCK)]));
    page.append(&heading("Desempenho"));
    page.append(&section(&[
        switch_row(&HARDWARE_ACCELERATION),
        gpu_diagnostics_row(),
    ]));
    page
}

fn heading(text: &str) -> Label {
    let label = Label::builder().label(text).halign(Align::Start).build();
    label.add_css_class("heading");
    label
}

fn section(rows: &[GtkBox]) -> Frame {
    let list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    for row in rows {
        list.append(row);
    }
    Frame::builder().child(&list).build()
}

fn switch_row(setting: &Setting) -> GtkBox {
    let switch = Switch::builder()
        .action_name(setting.action.detailed_name())
        .valign(Align::Center)
        .build();
    row(setting.title, setting.description, &switch)
}

fn gpu_diagnostics_row() -> GtkBox {
    let button = Button::builder()
        .label("Abrir")
        .valign(Align::Center)
        .build();
    button.connect_clicked(open_gpu_diagnostics);
    row(
        "Diagnóstico da GPU",
        "Mostra o renderizador, o uso de DMA-BUF e a decodificação de vídeo por hardware (webkit://gpu).",
        &button,
    )
}

fn open_gpu_diagnostics(button: &Button) {
    let Some(app) = button
        .root()
        .and_downcast::<Window>()
        .and_then(|w| w.application())
    else {
        return;
    };
    let Some(browser) = browser_window(&app) else {
        return;
    };
    let action = BrowserAction::OpenInNewTab.detailed_name();
    if let Err(error) = browser.activate_action(&action, Some(&GPU_DIAGNOSTICS_URI.to_variant())) {
        eprintln!("Não foi possível abrir o diagnóstico da GPU: {error}");
    }
    browser.present();
}

fn row(title: &str, description: &str, control: &impl IsA<Widget>) -> GtkBox {
    let texts = GtkBox::new(Orientation::Vertical, 4);
    texts.set_hexpand(true);
    texts.append(&Label::builder().label(title).halign(Align::Start).build());
    let details = Label::builder()
        .label(description)
        .halign(Align::Start)
        .xalign(0.0)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .build();
    details.add_css_class("dim-label");
    texts.append(&details);
    let row = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(SPACING)
        .margin_top(ROW_MARGIN)
        .margin_bottom(ROW_MARGIN)
        .margin_start(ROW_MARGIN)
        .margin_end(ROW_MARGIN)
        .build();
    row.append(&texts);
    row.append(control);
    row
}
