use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, Orientation, Popover, pango};

use crate::ui::actions::{ActionSpec, BrowserAction};

const SPACING: i32 = 4;
const MAX_CHARS: i32 = 48;

pub fn build(title: &str, uri: &str) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, SPACING);
    row.append(&open_button(title, uri));
    row
}

pub fn add_remove_button<F: Fn() + 'static>(row: &GtkBox, tooltip: &str, on_remove: F) {
    let button = Button::builder()
        .icon_name("user-trash-symbolic")
        .tooltip_text(tooltip)
        .valign(Align::Center)
        .build();
    button.add_css_class("flat");
    button.connect_clicked(move |_| on_remove());
    row.append(&button);
}

fn open_button(title: &str, uri: &str) -> Button {
    let labels = GtkBox::new(Orientation::Vertical, 0);
    labels.append(&ellipsized(title, None));
    labels.append(&ellipsized(uri, Some("dim-label")));
    let button = Button::builder()
        .child(&labels)
        .hexpand(true)
        .tooltip_text(uri)
        .build();
    button.add_css_class("flat");
    let uri = uri.to_owned();
    button.connect_clicked(move |button| open_in_current_tab(button, &uri));
    button
}

fn ellipsized(text: &str, css_class: Option<&str>) -> Label {
    let label = Label::builder()
        .label(text)
        .halign(Align::Start)
        .ellipsize(pango::EllipsizeMode::End)
        .max_width_chars(MAX_CHARS)
        .build();
    if let Some(css_class) = css_class {
        label.add_css_class(css_class);
    }
    label
}

fn open_in_current_tab(button: &Button, uri: &str) {
    let action = BrowserAction::OpenAddress.detailed_name();
    if let Err(error) = button.activate_action(&action, Some(&uri.to_variant())) {
        eprintln!("Não foi possível abrir {uri}: {error}");
    }
    if let Some(popover) = button
        .ancestor(Popover::static_type())
        .and_downcast::<Popover>()
    {
        popover.popdown();
    }
}
