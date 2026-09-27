use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Label, Orientation, glib, pango};
use webkit6::WebView;
use webkit6::prelude::*;

use crate::ui::title::tab_title;

const SPACING: i32 = 4;
const MAX_TITLE_CHARS: i32 = 24;

pub struct TabLabel {
    container: GtkBox,
    close_button: Button,
}

impl TabLabel {
    pub fn new(webview: &WebView) -> Self {
        let title = Label::builder()
            .label(tab_title(None, None))
            .ellipsize(pango::EllipsizeMode::End)
            .max_width_chars(MAX_TITLE_CHARS)
            .build();
        let close_button = Button::builder()
            .icon_name("window-close-symbolic")
            .tooltip_text("Fechar aba (Ctrl+W)")
            .build();
        close_button.add_css_class("flat");
        let container = GtkBox::new(Orientation::Horizontal, SPACING);
        container.append(&title);
        container.append(&close_button);
        bind_title(webview, &title);
        Self {
            container,
            close_button,
        }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    pub fn connect_close<F: Fn() + 'static>(&self, callback: F) {
        self.close_button.connect_clicked(move |_| callback());
    }
}

fn bind_title(webview: &WebView, label: &Label) {
    for property in ["title", "uri"] {
        webview.connect_notify_local(
            Some(property),
            glib::clone!(
                #[weak]
                label,
                move |webview, _| {
                    let title = tab_title(webview.title().as_deref(), webview.uri().as_deref());
                    label.set_tooltip_text(Some(&title));
                    label.set_text(&title);
                }
            ),
        );
    }
}
