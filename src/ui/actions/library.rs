use super::ActionSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryAction {
    ToggleBookmark,
    ShowBookmarks,
}

impl LibraryAction {
    pub const ALL: [Self; 2] = [Self::ToggleBookmark, Self::ShowBookmarks];
}

impl ActionSpec for LibraryAction {
    const SCOPE: &'static str = "win";

    fn name(self) -> &'static str {
        match self {
            Self::ToggleBookmark => "toggle-bookmark",
            Self::ShowBookmarks => "show-bookmarks",
        }
    }

    fn accels(self) -> &'static [&'static str] {
        match self {
            Self::ToggleBookmark => &["<Control>d"],
            Self::ShowBookmarks => &["<Control><Shift>o"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionSpec, LibraryAction};

    #[test]
    fn bookmark_actions_use_window_scope() {
        assert_eq!(
            LibraryAction::ToggleBookmark.detailed_name(),
            "win.toggle-bookmark"
        );
        assert_eq!(
            LibraryAction::ShowBookmarks.detailed_name(),
            "win.show-bookmarks"
        );
    }

    #[test]
    fn bookmark_shortcuts_are_registered() {
        assert_eq!(LibraryAction::ToggleBookmark.accels(), ["<Control>d"]);
        assert_eq!(LibraryAction::ShowBookmarks.accels(), ["<Control><Shift>o"]);
    }
}
