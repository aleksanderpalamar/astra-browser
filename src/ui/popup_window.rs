use gtk::prelude::*;
use gtk::{Box as GtkBox, Label, Orientation, Window, glib, pango};
use webkit6::WebView;
use webkit6::prelude::*;

use crate::ui::title::tab_title;

const ADDRESS_MARGIN: i32 = 6;

pub fn present(popup: &WebView, opener: Option<&WebView>, width: i32, height: i32) {
    let window = Window::builder()
        .default_width(width)
        .default_height(height)
        .child(&layout(popup))
        .build();
    if let Some(parent) = opener.and_then(WidgetExt::root).and_downcast::<Window>() {
        window.set_transient_for(Some(&parent));
        window.set_application(parent.application().as_ref());
    }
    bind_title(popup, &window);
    popup.connect_close(glib::clone!(
        #[weak]
        window,
        move |_| window.close()
    ));
    window.present();
}

fn layout(popup: &WebView) -> GtkBox {
    let address = Label::builder()
        .ellipsize(pango::EllipsizeMode::Middle)
        .xalign(0.0)
        .selectable(true)
        .margin_top(ADDRESS_MARGIN)
        .margin_bottom(ADDRESS_MARGIN)
        .margin_start(ADDRESS_MARGIN)
        .margin_end(ADDRESS_MARGIN)
        .build();
    address.add_css_class("dim-label");
    show_address(popup, &address);
    popup.connect_uri_notify(glib::clone!(
        #[weak]
        address,
        move |popup| show_address(popup, &address)
    ));
    let layout = GtkBox::new(Orientation::Vertical, 0);
    layout.append(&address);
    layout.append(popup);
    layout
}

fn show_address(popup: &WebView, address: &Label) {
    let uri = popup.uri().unwrap_or_default();
    address.set_text(&uri);
    address.set_tooltip_text(Some(&uri));
}

fn bind_title(popup: &WebView, window: &Window) {
    let update = glib::clone!(
        #[weak]
        window,
        move |popup: &WebView| {
            let title = tab_title(popup.title().as_deref(), popup.uri().as_deref());
            window.set_title(Some(&title));
        }
    );
    update(popup);
    popup.connect_title_notify(update);
}
