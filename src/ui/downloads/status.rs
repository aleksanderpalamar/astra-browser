use std::path::Path;

use gtk::glib;
use webkit6::DownloadError;

pub const FINISHED_TEXT: &str = "Concluído";
pub const PREPARING_TEXT: &str = "Preparando download…";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Running,
    Finished,
    Failed,
}

pub fn progress_text(fraction: f64) -> String {
    let percent = (fraction.clamp(0.0, 1.0) * 100.0).round();
    format!("{percent}%")
}

pub fn failure_text(error: &glib::Error) -> String {
    if error.matches(DownloadError::CancelledByUser) {
        return "Cancelado".to_owned();
    }
    format!("Falhou: {}", error.message())
}

pub fn file_name(destination: Option<&str>) -> String {
    destination
        .and_then(|path| Path::new(path).file_name())
        .map_or_else(
            || PREPARING_TEXT.to_owned(),
            |name| name.to_string_lossy().into_owned(),
        )
}

#[cfg(test)]
mod tests {
    use gtk::glib;
    use webkit6::DownloadError;

    use super::{PREPARING_TEXT, failure_text, file_name, progress_text};

    #[test]
    fn progress_is_a_rounded_percentage() {
        assert_eq!(progress_text(0.0), "0%");
        assert_eq!(progress_text(0.426), "43%");
        assert_eq!(progress_text(1.0), "100%");
    }

    #[test]
    fn progress_is_clamped() {
        assert_eq!(progress_text(-0.5), "0%");
        assert_eq!(progress_text(1.7), "100%");
    }

    #[test]
    fn cancellation_is_not_reported_as_failure() {
        let cancelled = glib::Error::new(DownloadError::CancelledByUser, "cancelado");
        assert_eq!(failure_text(&cancelled), "Cancelado");
        let network = glib::Error::new(DownloadError::Network, "sem conexão");
        assert_eq!(failure_text(&network), "Falhou: sem conexão");
    }

    #[test]
    fn file_name_comes_from_destination() {
        assert_eq!(
            file_name(Some("/home/u/Downloads/foto (1).png")),
            "foto (1).png"
        );
        assert_eq!(file_name(None), PREPARING_TEXT);
    }
}
