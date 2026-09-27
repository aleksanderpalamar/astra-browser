use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{
    Align, Box as GtkBox, Button, FileLauncher, Label, Orientation, ProgressBar, Window, gio, glib,
    pango,
};
use webkit6::Download;

use super::status::{self, Phase};

const SPACING: i32 = 4;
const MAX_NAME_CHARS: i32 = 40;

struct RowWidgets {
    name: Label,
    state: Label,
    progress: ProgressBar,
    cancel: Button,
    open: Button,
}

pub fn build(download: &Download) -> GtkBox {
    let widgets = RowWidgets {
        name: text_label(status::PREPARING_TEXT, None),
        state: text_label(&status::progress_text(0.0), Some("dim-label")),
        progress: ProgressBar::new(),
        cancel: icon_button("process-stop-symbolic", "Cancelar download"),
        open: icon_button("document-open-symbolic", "Abrir arquivo"),
    };
    widgets.open.set_visible(false);
    bind_progress(download, &widgets);
    bind_completion(download, &widgets);
    bind_buttons(download, &widgets);
    layout(&widgets)
}

fn layout(widgets: &RowWidgets) -> GtkBox {
    let details = GtkBox::new(Orientation::Vertical, SPACING);
    details.set_hexpand(true);
    details.append(&widgets.name);
    details.append(&widgets.progress);
    details.append(&widgets.state);
    let row = GtkBox::new(Orientation::Horizontal, SPACING);
    row.set_margin_top(SPACING);
    row.set_margin_bottom(SPACING);
    row.append(&details);
    row.append(&widgets.cancel);
    row.append(&widgets.open);
    row
}

fn bind_progress(download: &Download, widgets: &RowWidgets) {
    let RowWidgets {
        name,
        state,
        progress,
        ..
    } = widgets;
    download.connect_destination_notify(glib::clone!(
        #[weak]
        name,
        move |download| name.set_text(&status::file_name(download.destination().as_deref()))
    ));
    download.connect_estimated_progress_notify(glib::clone!(
        #[weak]
        state,
        #[weak]
        progress,
        move |download| {
            let fraction = download.estimated_progress();
            progress.set_fraction(fraction);
            state.set_text(&status::progress_text(fraction));
        }
    ));
}

fn bind_completion(download: &Download, widgets: &RowWidgets) {
    let RowWidgets {
        state,
        progress,
        cancel,
        open,
        ..
    } = widgets;
    let phase = Rc::new(Cell::new(Phase::Running));
    download.connect_failed(glib::clone!(
        #[weak]
        state,
        #[weak]
        cancel,
        #[strong]
        phase,
        move |_, error| {
            phase.set(Phase::Failed);
            state.set_text(&status::failure_text(error));
            cancel.set_visible(false);
        }
    ));
    download.connect_finished(glib::clone!(
        #[weak]
        state,
        #[weak]
        progress,
        #[weak]
        cancel,
        #[weak]
        open,
        move |_| {
            if phase.get() == Phase::Failed {
                return;
            }
            phase.set(Phase::Finished);
            state.set_text(status::FINISHED_TEXT);
            progress.set_fraction(1.0);
            cancel.set_visible(false);
            open.set_visible(true);
        }
    ));
}

fn bind_buttons(download: &Download, widgets: &RowWidgets) {
    widgets.cancel.connect_clicked(glib::clone!(
        #[strong]
        download,
        move |_| download.cancel()
    ));
    widgets.open.connect_clicked(glib::clone!(
        #[strong]
        download,
        move |button| open_file(button, &download)
    ));
}

fn open_file(button: &Button, download: &Download) {
    let Some(destination) = download.destination() else {
        return;
    };
    let launcher = FileLauncher::new(Some(&gio::File::for_path(destination.as_str())));
    let parent = button.root().and_downcast::<Window>();
    launcher.launch(parent.as_ref(), gio::Cancellable::NONE, move |result| {
        if let Err(error) = result {
            eprintln!("Não foi possível abrir {destination}: {error}");
        }
    });
}

fn text_label(text: &str, css_class: Option<&str>) -> Label {
    let label = Label::builder()
        .label(text)
        .halign(Align::Start)
        .ellipsize(pango::EllipsizeMode::Middle)
        .max_width_chars(MAX_NAME_CHARS)
        .build();
    if let Some(css_class) = css_class {
        label.add_css_class(css_class);
    }
    label
}

fn icon_button(icon: &str, tooltip: &str) -> Button {
    let button = Button::builder()
        .icon_name(icon)
        .tooltip_text(tooltip)
        .valign(Align::Center)
        .build();
    button.add_css_class("flat");
    button
}
