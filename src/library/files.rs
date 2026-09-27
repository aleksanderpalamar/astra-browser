use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

use gtk::glib;

const APP_DIRECTORY: &str = "astra-browser";

pub fn data_path(file_name: &str) -> PathBuf {
    glib::user_data_dir().join(APP_DIRECTORY).join(file_name)
}

pub fn config_path(file_name: &str) -> PathBuf {
    glib::user_config_dir().join(APP_DIRECTORY).join(file_name)
}

pub fn read_or_empty(path: &Path) -> io::Result<String> {
    match fs::read_to_string(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(String::new()),
        result => result,
    }
}

pub fn write_atomically(path: &Path, contents: &str) -> io::Result<()> {
    ensure_parent(path)?;
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents)?;
    fs::rename(&temporary, path)
}

pub fn append_line(path: &Path, line: &str) -> io::Result<()> {
    ensure_parent(path)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")
}

fn ensure_parent(path: &Path) -> io::Result<()> {
    match path.parent() {
        Some(parent) => fs::create_dir_all(parent),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{append_line, read_or_empty, write_atomically};

    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("astra-browser-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn missing_file_reads_as_empty() {
        let dir = scratch_dir("missing");
        assert_eq!(
            read_or_empty(&dir.join("nada.tsv")).ok(),
            Some(String::new())
        );
    }

    #[test]
    fn writes_creating_parent_directories() {
        let dir = scratch_dir("write");
        let path = dir.join("sub").join("dados.tsv");
        assert!(write_atomically(&path, "linha\n").is_ok());
        assert_eq!(read_or_empty(&path).ok().as_deref(), Some("linha\n"));
        assert!(write_atomically(&path, "nova\n").is_ok());
        assert_eq!(read_or_empty(&path).ok().as_deref(), Some("nova\n"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn appends_lines_to_new_and_existing_files() {
        let dir = scratch_dir("append");
        let path = dir.join("historico.tsv");
        assert!(append_line(&path, "primeira").is_ok());
        assert!(append_line(&path, "segunda").is_ok());
        assert_eq!(
            read_or_empty(&path).ok().as_deref(),
            Some("primeira\nsegunda\n")
        );
        let _ = fs::remove_dir_all(dir);
    }
}
