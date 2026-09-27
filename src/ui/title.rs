pub const APP_NAME: &str = "Rust Browser";
pub const NEW_TAB_TITLE: &str = "Nova aba";

pub fn window_title(page_title: Option<&str>) -> String {
    match page_title.map(str::trim).filter(|title| !title.is_empty()) {
        Some(title) => format!("{title} — {APP_NAME}"),
        None => APP_NAME.to_owned(),
    }
}

pub fn tab_title(page_title: Option<&str>, uri: Option<&str>) -> String {
    [page_title, uri]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|candidate| !candidate.is_empty())
        .unwrap_or(NEW_TAB_TITLE)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{APP_NAME, NEW_TAB_TITLE, tab_title, window_title};

    #[test]
    fn appends_app_name_to_page_title() {
        assert_eq!(
            window_title(Some("GitHub · Build and ship software")),
            "GitHub · Build and ship software — Rust Browser"
        );
    }

    #[test]
    fn falls_back_to_app_name_without_page_title() {
        assert_eq!(window_title(None), APP_NAME);
        assert_eq!(window_title(Some("")), APP_NAME);
        assert_eq!(window_title(Some("   ")), APP_NAME);
    }

    #[test]
    fn trims_page_title() {
        assert_eq!(
            window_title(Some("  DuckDuckGo \n")),
            "DuckDuckGo — Rust Browser"
        );
    }

    #[test]
    fn tab_prefers_page_title() {
        assert_eq!(
            tab_title(Some(" GitHub "), Some("https://github.com/")),
            "GitHub"
        );
    }

    #[test]
    fn tab_falls_back_to_uri_while_title_is_unknown() {
        assert_eq!(
            tab_title(Some(""), Some("https://github.com/")),
            "https://github.com/"
        );
        assert_eq!(
            tab_title(None, Some("https://github.com/")),
            "https://github.com/"
        );
    }

    #[test]
    fn empty_tab_is_named_new_tab() {
        assert_eq!(tab_title(None, None), NEW_TAB_TITLE);
        assert_eq!(tab_title(Some(" "), Some("")), NEW_TAB_TITLE);
    }
}
