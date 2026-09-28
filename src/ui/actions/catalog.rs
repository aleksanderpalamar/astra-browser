use gtk::glib;

use super::ActionSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserAction {
    Back,
    Forward,
    Reload,
    Home,
    FocusAddress,
    OpenAddress,
    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,
    ToggleInspector,
}

impl BrowserAction {
    pub const ALL: [Self; 11] = [
        Self::Back,
        Self::Forward,
        Self::Reload,
        Self::Home,
        Self::FocusAddress,
        Self::OpenAddress,
        Self::NewTab,
        Self::CloseTab,
        Self::NextTab,
        Self::PreviousTab,
        Self::ToggleInspector,
    ];

    pub(super) fn depends_on_history(self) -> bool {
        matches!(self, Self::Back | Self::Forward)
    }
}

impl ActionSpec for BrowserAction {
    const SCOPE: &'static str = "win";

    fn name(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Forward => "forward",
            Self::Reload => "reload",
            Self::Home => "home",
            Self::FocusAddress => "focus-address",
            Self::OpenAddress => "open-address",
            Self::NewTab => "new-tab",
            Self::CloseTab => "close-tab",
            Self::NextTab => "next-tab",
            Self::PreviousTab => "previous-tab",
            Self::ToggleInspector => "toggle-inspector",
        }
    }

    fn accels(self) -> &'static [&'static str] {
        match self {
            Self::Back => &["<Alt>Left"],
            Self::Forward => &["<Alt>Right"],
            Self::Reload => &["<Control>r", "F5"],
            Self::Home => &["<Alt>Home"],
            Self::FocusAddress => &["<Control>l"],
            Self::OpenAddress => &[],
            Self::NewTab => &["<Control>t"],
            Self::CloseTab => &["<Control>w"],
            Self::NextTab => &["<Control>Tab", "<Control>Page_Down"],
            Self::PreviousTab => &["<Control><Shift>Tab", "<Control>Page_Up"],
            Self::ToggleInspector => &["F12", "<Control><Shift>i"],
        }
    }

    fn parameter_type(self) -> Option<&'static glib::VariantTy> {
        match self {
            Self::OpenAddress => Some(glib::VariantTy::STRING),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionSpec, BrowserAction};

    #[test]
    fn detailed_names_use_window_scope() {
        assert_eq!(BrowserAction::Back.detailed_name(), "win.back");
        assert_eq!(BrowserAction::NewTab.detailed_name(), "win.new-tab");
    }

    #[test]
    fn action_names_are_unique() {
        let mut names = BrowserAction::ALL.map(BrowserAction::name);
        names.sort_unstable();
        assert!(names.windows(2).all(|pair| pair[0] != pair[1]));
    }

    #[test]
    fn only_back_and_forward_depend_on_history() {
        let dependent: Vec<_> = BrowserAction::ALL
            .into_iter()
            .filter(|action| action.depends_on_history())
            .collect();
        assert_eq!(dependent, [BrowserAction::Back, BrowserAction::Forward]);
    }

    #[test]
    fn required_shortcuts_are_registered() {
        assert!(BrowserAction::FocusAddress.accels().contains(&"<Control>l"));
        assert!(BrowserAction::Reload.accels().contains(&"<Control>r"));
        assert!(BrowserAction::Reload.accels().contains(&"F5"));
        assert!(BrowserAction::Back.accels().contains(&"<Alt>Left"));
        assert!(BrowserAction::Forward.accels().contains(&"<Alt>Right"));
    }

    #[test]
    fn tab_shortcuts_are_registered() {
        assert!(BrowserAction::NewTab.accels().contains(&"<Control>t"));
        assert!(BrowserAction::CloseTab.accels().contains(&"<Control>w"));
        assert!(BrowserAction::NextTab.accels().contains(&"<Control>Tab"));
        assert!(
            BrowserAction::PreviousTab
                .accels()
                .contains(&"<Control>Page_Up")
        );
    }

    #[test]
    fn inspector_uses_the_usual_browser_shortcuts() {
        assert_eq!(
            BrowserAction::ToggleInspector.detailed_name(),
            "win.toggle-inspector"
        );
        assert!(BrowserAction::ToggleInspector.accels().contains(&"F12"));
        assert!(
            BrowserAction::ToggleInspector
                .accels()
                .contains(&"<Control><Shift>i")
        );
    }

    #[test]
    fn only_open_address_takes_a_parameter() {
        for action in BrowserAction::ALL {
            let expects_parameter = action == BrowserAction::OpenAddress;
            assert_eq!(action.parameter_type().is_some(), expects_parameter);
        }
    }
}
