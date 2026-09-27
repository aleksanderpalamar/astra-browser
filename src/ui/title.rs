pub const APP_NAME: &str = "Rust Browser";

pub fn window_title(page_title: Option<&str>) -> String {
    match page_title.map(str::trim).filter(|title| !title.is_empty()) {
        Some(title) => format!("{title} — {APP_NAME}"),
        None => APP_NAME.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{APP_NAME, window_title};

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
}
