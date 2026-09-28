use super::ActionSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAction {
    NewWindow,
    NewPrivateWindow,
    ToggleAdBlock,
    ShowPreferences,
    Quit,
}

impl AppAction {
    pub const ALL: [Self; 5] = [
        Self::NewWindow,
        Self::NewPrivateWindow,
        Self::ToggleAdBlock,
        Self::ShowPreferences,
        Self::Quit,
    ];
}

impl ActionSpec for AppAction {
    const SCOPE: &'static str = "app";

    fn name(self) -> &'static str {
        match self {
            Self::NewWindow => "new-window",
            Self::NewPrivateWindow => "new-private-window",
            Self::ToggleAdBlock => "toggle-adblock",
            Self::ShowPreferences => "show-preferences",
            Self::Quit => "quit",
        }
    }

    fn accels(self) -> &'static [&'static str] {
        match self {
            Self::NewWindow => &["<Control>n"],
            Self::NewPrivateWindow => &["<Control><Shift>p"],
            Self::ToggleAdBlock => &[],
            Self::ShowPreferences => &["<Control>comma"],
            Self::Quit => &["<Control>q"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionSpec, AppAction};

    #[test]
    fn app_actions_use_application_scope() {
        assert_eq!(AppAction::Quit.detailed_name(), "app.quit");
        assert_eq!(
            AppAction::NewPrivateWindow.detailed_name(),
            "app.new-private-window"
        );
    }

    #[test]
    fn window_shortcuts_are_registered() {
        assert_eq!(AppAction::Quit.accels(), ["<Control>q"]);
        assert_eq!(AppAction::NewWindow.accels(), ["<Control>n"]);
        assert_eq!(AppAction::NewPrivateWindow.accels(), ["<Control><Shift>p"]);
    }

    #[test]
    fn preferences_use_the_gnome_shortcut() {
        assert_eq!(
            AppAction::ShowPreferences.detailed_name(),
            "app.show-preferences"
        );
        assert_eq!(AppAction::ShowPreferences.accels(), ["<Control>comma"]);
    }
}
