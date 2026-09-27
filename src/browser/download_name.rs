use std::ffi::OsStr;
use std::path::{Path, PathBuf};

const FALLBACK_NAME: &str = "download";
const MAX_ATTEMPTS: u32 = 1000;

pub fn unique_destination(
    directory: &Path,
    suggested: &str,
    exists: impl Fn(&Path) -> bool,
) -> PathBuf {
    let name = sanitized_name(suggested);
    let (stem, extension) = split_extension(name);
    (0..MAX_ATTEMPTS)
        .map(|attempt| directory.join(numbered(stem, extension, attempt)))
        .find(|candidate| !exists(candidate))
        .unwrap_or_else(|| directory.join(numbered(stem, extension, MAX_ATTEMPTS)))
}

fn sanitized_name(suggested: &str) -> &str {
    Path::new(suggested.trim())
        .file_name()
        .and_then(OsStr::to_str)
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(FALLBACK_NAME)
}

fn split_extension(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(dot) if dot > 0 => name.split_at(dot),
        _ => (name, ""),
    }
}

fn numbered(stem: &str, extension: &str, attempt: u32) -> String {
    match attempt {
        0 => format!("{stem}{extension}"),
        _ => format!("{stem} ({attempt}){extension}"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::unique_destination;

    fn destination(suggested: &str, taken: &[&str]) -> PathBuf {
        let directory = Path::new("/downloads");
        unique_destination(directory, suggested, |path| {
            taken.iter().any(|name| directory.join(name) == path)
        })
    }

    #[test]
    fn keeps_free_names() {
        assert_eq!(
            destination("relatorio.pdf", &[]),
            Path::new("/downloads/relatorio.pdf")
        );
    }

    #[test]
    fn numbers_names_already_taken() {
        assert_eq!(
            destination("foto.png", &["foto.png"]),
            Path::new("/downloads/foto (1).png")
        );
        assert_eq!(
            destination("foto.png", &["foto.png", "foto (1).png"]),
            Path::new("/downloads/foto (2).png")
        );
    }

    #[test]
    fn numbers_names_without_extension_and_hidden_files() {
        assert_eq!(
            destination("LEIAME", &["LEIAME"]),
            Path::new("/downloads/LEIAME (1)")
        );
        assert_eq!(
            destination(".bashrc", &[".bashrc"]),
            Path::new("/downloads/.bashrc (1)")
        );
    }

    #[test]
    fn numbers_only_the_last_extension() {
        assert_eq!(
            destination("fonte.tar.gz", &["fonte.tar.gz"]),
            Path::new("/downloads/fonte.tar (1).gz")
        );
    }

    #[test]
    fn strips_directories_from_suggested_names() {
        assert_eq!(
            destination("../../etc/passwd", &[]),
            Path::new("/downloads/passwd")
        );
        assert_eq!(
            destination("/tmp/x.zip", &[]),
            Path::new("/downloads/x.zip")
        );
    }

    #[test]
    fn falls_back_when_no_name_is_suggested() {
        assert_eq!(destination("", &[]), Path::new("/downloads/download"));
        assert_eq!(destination("..", &[]), Path::new("/downloads/download"));
        assert_eq!(destination("   ", &[]), Path::new("/downloads/download"));
    }
}
