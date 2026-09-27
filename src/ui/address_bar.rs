use gtk::prelude::*;
use gtk::{Entry, EntryIconPosition, InputPurpose, glib};

use crate::library::bookmarks::BookmarkState;
use crate::ui::actions::{ActionSpec, BrowserAction, LibraryAction};

const PLACEHOLDER: &str = "Pesquise ou digite um endereço";

pub fn build() -> Entry {
    let entry = Entry::builder()
        .hexpand(true)
        .input_purpose(InputPurpose::Url)
        .placeholder_text(PLACEHOLDER)
        .build();
    entry.connect_activate(|entry| {
        let input = entry.text().to_variant();
        activate(entry, BrowserAction::OpenAddress, Some(&input));
    });
    entry.connect_icon_press(|entry, position| {
        if position == EntryIconPosition::Secondary {
            activate(entry, LibraryAction::ToggleBookmark, None);
        }
    });
    show_bookmark_state(&entry, BookmarkState::NotBookmarked);
    entry
}

pub fn show_bookmark_state(entry: &Entry, state: BookmarkState) {
    let (icon, tooltip) = star(state);
    entry.set_secondary_icon_name(Some(icon));
    entry.set_secondary_icon_tooltip_text(Some(tooltip));
}

fn star(state: BookmarkState) -> (&'static str, &'static str) {
    match state {
        BookmarkState::Bookmarked => ("starred-symbolic", "Remover dos favoritos (Ctrl+D)"),
        BookmarkState::NotBookmarked => {
            ("non-starred-symbolic", "Adicionar aos favoritos (Ctrl+D)")
        }
    }
}

fn activate(entry: &Entry, action: impl ActionSpec, parameter: Option<&glib::Variant>) {
    let name = action.detailed_name();
    if let Err(error) = entry.activate_action(&name, parameter) {
        eprintln!("Não foi possível executar {name}: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::{BookmarkState, star};

    #[test]
    fn star_reflects_bookmark_state() {
        assert_eq!(star(BookmarkState::Bookmarked).0, "starred-symbolic");
        assert_eq!(star(BookmarkState::NotBookmarked).0, "non-starred-symbolic");
    }

    #[test]
    fn star_tooltip_explains_the_next_action() {
        assert!(star(BookmarkState::Bookmarked).1.starts_with("Remover"));
        assert!(
            star(BookmarkState::NotBookmarked)
                .1
                .starts_with("Adicionar")
        );
    }
}
