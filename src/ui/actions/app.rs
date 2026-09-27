use super::ActionSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAction {
    NewWindow,
    NewPrivateWindow,
    ToggleAdBlock,
    Quit,
}

impl AppAction {
    pub const ALL: [Self; 4] = [
        Self::NewWindow,
        Self::NewPrivateWindow,
        Self::ToggleAdBlock,
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
            Self::Quit => "quit",
        }
    }

    fn accels(self) -> &'static [&'static str] {
        match self {
            Self::NewWindow => &["<Control>n"],
            Self::NewPrivateWindow => &["<Control><Shift>p"],
            Self::ToggleAdBlock => &[],
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
}
