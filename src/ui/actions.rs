use gtk::prelude::*;
use gtk::{ApplicationWindow, gio, glib};
use webkit6::WebView;
use webkit6::prelude::*;

use crate::browser::navigation::Navigator;
use crate::ui::toolbar::Toolbar;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserAction {
    Back,
    Forward,
    Reload,
    Home,
    FocusAddress,
    OpenAddress,
}

impl BrowserAction {
    pub const ALL: [Self; 6] = [
        Self::Back,
        Self::Forward,
        Self::Reload,
        Self::Home,
        Self::FocusAddress,
        Self::OpenAddress,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Forward => "forward",
            Self::Reload => "reload",
            Self::Home => "home",
            Self::FocusAddress => "focus-address",
            Self::OpenAddress => "open-address",
        }
    }

    pub fn detailed_name(self) -> String {
        format!("win.{}", self.name())
    }

    pub fn accels(self) -> &'static [&'static str] {
        match self {
            Self::Back => &["<Alt>Left"],
            Self::Forward => &["<Alt>Right"],
            Self::Reload => &["<Control>r", "F5"],
            Self::Home => &["<Alt>Home"],
            Self::FocusAddress => &["<Control>l"],
            Self::OpenAddress => &[],
        }
    }

    fn parameter_type(self) -> Option<&'static glib::VariantTy> {
        match self {
            Self::OpenAddress => Some(glib::VariantTy::STRING),
            _ => None,
        }
    }

    fn depends_on_history(self) -> bool {
        matches!(self, Self::Back | Self::Forward)
    }
}

#[derive(Clone)]
pub struct HistoryActions {
    back: gio::SimpleAction,
    forward: gio::SimpleAction,
}

impl HistoryActions {
    pub fn update(&self, webview: &WebView) {
        self.back.set_enabled(webview.can_go_back());
        self.forward.set_enabled(webview.can_go_forward());
    }
}

#[derive(Clone)]
struct Handler {
    navigator: Navigator,
    toolbar: Toolbar,
}

impl Handler {
    fn perform(&self, action: BrowserAction, parameter: Option<&glib::Variant>) {
        match action {
            BrowserAction::Back => self.navigator.back(),
            BrowserAction::Forward => self.navigator.forward(),
            BrowserAction::Reload => self.navigator.reload(),
            BrowserAction::Home => self.navigator.home(),
            BrowserAction::FocusAddress => self.toolbar.focus_address(),
            BrowserAction::OpenAddress => {
                if let Some(input) = parameter.and_then(glib::Variant::str) {
                    self.navigator.open(input);
                }
            }
        }
    }
}

pub fn install(
    window: &ApplicationWindow,
    navigator: Navigator,
    toolbar: Toolbar,
) -> HistoryActions {
    let handler = Handler { navigator, toolbar };
    let history = HistoryActions {
        back: register(window, BrowserAction::Back, &handler),
        forward: register(window, BrowserAction::Forward, &handler),
    };
    BrowserAction::ALL
        .into_iter()
        .filter(|action| !action.depends_on_history())
        .for_each(|action| {
            register(window, action, &handler);
        });
    history
}

fn register(
    window: &ApplicationWindow,
    action: BrowserAction,
    handler: &Handler,
) -> gio::SimpleAction {
    let simple = gio::SimpleAction::new(action.name(), action.parameter_type());
    let handler = handler.clone();
    simple.connect_activate(move |_, parameter| handler.perform(action, parameter));
    window.add_action(&simple);
    simple
}

#[cfg(test)]
mod tests {
    use super::BrowserAction;

    #[test]
    fn detailed_names_use_window_scope() {
        assert_eq!(BrowserAction::Back.detailed_name(), "win.back");
        assert_eq!(
            BrowserAction::FocusAddress.detailed_name(),
            "win.focus-address"
        );
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
    fn only_open_address_takes_a_parameter() {
        for action in BrowserAction::ALL {
            let expects_parameter = action == BrowserAction::OpenAddress;
            assert_eq!(action.parameter_type().is_some(), expects_parameter);
        }
    }
}
