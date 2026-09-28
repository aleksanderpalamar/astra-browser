use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Frame, Label, ListBox, Orientation,
    SelectionMode, Switch, Window, pango,
};

use crate::ui::actions::{ActionSpec, AppAction};

const WINDOW_NAME: &str = "astra-preferences";
const MARGIN: i32 = 24;
const ROW_MARGIN: i32 = 12;
const SPACING: i32 = 12;

struct Setting {
    title: &'static str,
    description: &'static str,
    action: AppAction,
}

const PRIVACY: [Setting; 1] = [Setting {
    title: "Bloquear anúncios",
    description: "Bloqueia redes de anúncio e esconde anúncios nas páginas, inclusive no YouTube. Recarregue a página para ver o efeito.",
    action: AppAction::ToggleAdBlock,
}];

pub fn show(app: &Application) {
    if let Some(window) = existing(app) {
        window.present();
        return;
    }
    let window = Window::builder()
        .application(app)
        .title("Configurações")
        .default_width(560)
        .default_height(320)
        .child(&page())
        .build();
    window.set_widget_name(WINDOW_NAME);
    if let Some(browser) = app
        .windows()
        .into_iter()
        .find(|w| w.is::<ApplicationWindow>())
    {
        window.set_transient_for(Some(&browser));
    }
    window.present();
}

fn existing(app: &Application) -> Option<Window> {
    app.windows()
        .into_iter()
        .find(|window| window.widget_name() == WINDOW_NAME)
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
    page.append(&section(&PRIVACY));
    page
}

fn heading(text: &str) -> Label {
    let label = Label::builder().label(text).halign(Align::Start).build();
    label.add_css_class("heading");
    label
}

fn section(settings: &[Setting]) -> Frame {
    let list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    for setting in settings {
        list.append(&switch_row(setting));
    }
    Frame::builder().child(&list).build()
}

fn switch_row(setting: &Setting) -> GtkBox {
    let texts = GtkBox::new(Orientation::Vertical, 4);
    texts.set_hexpand(true);
    texts.append(
        &Label::builder()
            .label(setting.title)
            .halign(Align::Start)
            .build(),
    );
    let description = Label::builder()
        .label(setting.description)
        .halign(Align::Start)
        .xalign(0.0)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .build();
    description.add_css_class("dim-label");
    texts.append(&description);
    let switch = Switch::builder()
        .action_name(setting.action.detailed_name())
        .valign(Align::Center)
        .build();
    let row = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(SPACING)
        .margin_top(ROW_MARGIN)
        .margin_bottom(ROW_MARGIN)
        .margin_start(ROW_MARGIN)
        .margin_end(ROW_MARGIN)
        .build();
    row.append(&texts);
    row.append(&switch);
    row
}
