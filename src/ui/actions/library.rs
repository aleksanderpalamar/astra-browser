use super::ActionSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryAction {
    ToggleBookmark,
    ShowBookmarks,
    ShowHistory,
    ShowDownloads,
}

impl LibraryAction {
    pub const ALL: [Self; 4] = [
        Self::ToggleBookmark,
        Self::ShowBookmarks,
        Self::ShowHistory,
        Self::ShowDownloads,
    ];
}

impl ActionSpec for LibraryAction {
    const SCOPE: &'static str = "win";

    fn name(self) -> &'static str {
        match self {
            Self::ToggleBookmark => "toggle-bookmark",
            Self::ShowBookmarks => "show-bookmarks",
            Self::ShowHistory => "show-history",
            Self::ShowDownloads => "show-downloads",
        }
    }

    fn accels(self) -> &'static [&'static str] {
        match self {
            Self::ToggleBookmark => &["<Control>d"],
            Self::ShowBookmarks => &["<Control><Shift>o"],
            Self::ShowHistory => &["<Control>h"],
            Self::ShowDownloads => &["<Control><Shift>y"],
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

    #[test]
    fn history_shortcut_is_registered() {
        assert_eq!(
            LibraryAction::ShowHistory.detailed_name(),
            "win.show-history"
        );
        assert_eq!(LibraryAction::ShowHistory.accels(), ["<Control>h"]);
    }

    #[test]
    fn downloads_shortcut_is_registered() {
        assert_eq!(
            LibraryAction::ShowDownloads.detailed_name(),
            "win.show-downloads"
        );
        assert_eq!(LibraryAction::ShowDownloads.accels(), ["<Control><Shift>y"]);
    }
}
