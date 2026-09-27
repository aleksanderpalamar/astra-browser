use std::path::PathBuf;

use gtk::glib::{KeyFile, KeyFileFlags};

use crate::library::files;

const GROUP: &str = "navegacao";
const ADBLOCK_KEY: &str = "bloquear-anuncios";

pub struct Preferences {
    path: PathBuf,
}

impl Preferences {
    pub fn open(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn adblock_enabled(&self) -> bool {
        adblock_enabled(&self.read())
    }

    pub fn set_adblock_enabled(&self, enabled: bool) {
        let contents = with_adblock(&self.read(), enabled);
        if let Err(error) = files::write_atomically(&self.path, &contents) {
            eprintln!(
                "Não foi possível salvar as preferências em {}: {error}",
                self.path.display()
            );
        }
    }

    fn read(&self) -> String {
        files::read_or_empty(&self.path).unwrap_or_else(|error| {
            eprintln!(
                "Não foi possível ler as preferências em {}: {error}",
                self.path.display()
            );
            String::new()
        })
    }
}

fn adblock_enabled(contents: &str) -> bool {
    parse(contents).boolean(GROUP, ADBLOCK_KEY).unwrap_or(true)
}

fn with_adblock(contents: &str, enabled: bool) -> String {
    let keys = parse(contents);
    keys.set_boolean(GROUP, ADBLOCK_KEY, enabled);
    keys.to_data().to_string()
}

fn parse(contents: &str) -> KeyFile {
    let keys = KeyFile::new();
    if let Err(error) = keys.load_from_data(contents, KeyFileFlags::KEEP_COMMENTS) {
        if !contents.trim().is_empty() {
            eprintln!("Preferências inválidas; usando os valores padrão: {error}");
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::{adblock_enabled, with_adblock};

    #[test]
    fn adblock_is_enabled_by_default() {
        assert!(adblock_enabled(""));
        assert!(adblock_enabled("[outro]\nchave=1\n"));
    }

    #[test]
    fn remembers_disabled_adblock() {
        let contents = with_adblock("", false);
        assert!(!adblock_enabled(&contents));
        assert!(adblock_enabled(&with_adblock(&contents, true)));
    }

    #[test]
    fn keeps_unrelated_settings() {
        let contents = with_adblock("[outro]\nchave=valor\n", false);
        assert!(contents.contains("chave=valor"));
        assert!(!adblock_enabled(&contents));
    }

    #[test]
    fn invalid_files_fall_back_to_defaults() {
        assert!(adblock_enabled("isto não é ini"));
    }
}
