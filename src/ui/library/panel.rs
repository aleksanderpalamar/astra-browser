use gtk::prelude::*;
use gtk::{
    Align, Box as GtkBox, Label, MenuButton, Orientation, PolicyType, Popover, ScrolledWindow,
    Widget, glib,
};

const SPACING: i32 = 6;
const MIN_WIDTH: i32 = 420;
const MAX_HEIGHT: i32 = 480;

pub struct PanelSpec {
    pub icon: &'static str,
    pub tooltip: &'static str,
    pub heading: &'static str,
    pub empty_text: &'static str,
}

#[derive(Clone)]
pub struct LibraryPanel {
    button: MenuButton,
    rows: GtkBox,
    empty: Label,
}

pub struct WeakLibraryPanel {
    button: glib::WeakRef<MenuButton>,
    rows: glib::WeakRef<GtkBox>,
    empty: glib::WeakRef<Label>,
}

impl glib::clone::Downgrade for LibraryPanel {
    type Weak = WeakLibraryPanel;

    fn downgrade(&self) -> WeakLibraryPanel {
        WeakLibraryPanel {
            button: ObjectExt::downgrade(&self.button),
            rows: ObjectExt::downgrade(&self.rows),
            empty: ObjectExt::downgrade(&self.empty),
        }
    }
}

impl glib::clone::Upgrade for WeakLibraryPanel {
    type Strong = LibraryPanel;

    fn upgrade(&self) -> Option<LibraryPanel> {
        Some(LibraryPanel {
            button: self.button.upgrade()?,
            rows: self.rows.upgrade()?,
            empty: self.empty.upgrade()?,
        })
    }
}

impl LibraryPanel {
    pub fn new(spec: &PanelSpec, header: Option<&Widget>, footer: Option<&Widget>) -> Self {
        let rows = GtkBox::new(Orientation::Vertical, 0);
        let empty = Label::builder()
            .label(spec.empty_text)
            .margin_top(SPACING * 2)
            .margin_bottom(SPACING * 2)
            .build();
        empty.add_css_class("dim-label");
        let content = GtkBox::new(Orientation::Vertical, SPACING);
        content.set_width_request(MIN_WIDTH);
        content.append(&heading(spec.heading));
        if let Some(header) = header {
            content.append(header);
        }
        content.append(&empty);
        content.append(&scroller(&rows));
        if let Some(footer) = footer {
            content.append(footer);
        }
        let button = MenuButton::builder()
            .icon_name(spec.icon)
            .tooltip_text(spec.tooltip)
            .popover(&Popover::builder().child(&content).build())
            .build();
        Self {
            button,
            rows,
            empty,
        }
    }

    pub fn button(&self) -> &MenuButton {
        &self.button
    }

    pub fn popup(&self) {
        self.button.popup();
    }

    pub fn connect_opening<F: Fn() + 'static>(&self, callback: F) {
        if let Some(popover) = self.button.popover() {
            popover.connect_show(move |_| callback());
        }
    }

    pub fn prepend_row(&self, row: &GtkBox) {
        self.empty.set_visible(false);
        self.rows.prepend(row);
    }

    pub fn show_rows(&self, rows: Vec<GtkBox>) {
        while let Some(child) = self.rows.first_child() {
            self.rows.remove(&child);
        }
        self.empty.set_visible(rows.is_empty());
        for row in &rows {
            self.rows.append(row);
        }
    }
}

fn heading(text: &str) -> Label {
    let label = Label::builder().label(text).halign(Align::Start).build();
    label.add_css_class("heading");
    label
}

fn scroller(rows: &GtkBox) -> ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .propagate_natural_height(true)
        .propagate_natural_width(true)
        .max_content_height(MAX_HEIGHT)
        .child(rows)
        .build()
}
