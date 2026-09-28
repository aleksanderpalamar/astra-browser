use std::path::PathBuf;

use gtk::glib::{KeyFile, KeyFileFlags};

use crate::library::files;

const GROUP: &str = "navegacao";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toggle {
    AdBlock,
    HardwareAcceleration,
}

impl Toggle {
    fn key(self) -> &'static str {
        match self {
            Self::AdBlock => "bloquear-anuncios",
            Self::HardwareAcceleration => "aceleracao-hardware",
        }
    }
}

pub struct Preferences {
    path: PathBuf,
}

impl Preferences {
    pub fn open(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn enabled(&self, toggle: Toggle) -> bool {
        enabled(&self.read(), toggle)
    }

    pub fn set_enabled(&self, toggle: Toggle, value: bool) {
        let contents = with_toggle(&self.read(), toggle, value);
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

fn enabled(contents: &str, toggle: Toggle) -> bool {
    parse(contents).boolean(GROUP, toggle.key()).unwrap_or(true)
}

fn with_toggle(contents: &str, toggle: Toggle, value: bool) -> String {
    let keys = parse(contents);
    keys.set_boolean(GROUP, toggle.key(), value);
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
    use super::{Toggle, enabled, with_toggle};

    #[test]
    fn toggles_are_enabled_by_default() {
        assert!(enabled("", Toggle::AdBlock));
        assert!(enabled("", Toggle::HardwareAcceleration));
        assert!(enabled("[outro]\nchave=1\n", Toggle::AdBlock));
    }

    #[test]
    fn remembers_disabled_toggles() {
        let contents = with_toggle("", Toggle::AdBlock, false);
        assert!(!enabled(&contents, Toggle::AdBlock));
        assert!(enabled(
            &with_toggle(&contents, Toggle::AdBlock, true),
            Toggle::AdBlock
        ));
    }

    #[test]
    fn toggles_are_independent() {
        let contents = with_toggle("", Toggle::HardwareAcceleration, false);
        assert!(!enabled(&contents, Toggle::HardwareAcceleration));
        assert!(enabled(&contents, Toggle::AdBlock));
    }

    #[test]
    fn keeps_unrelated_settings() {
        let contents = with_toggle("[outro]\nchave=valor\n", Toggle::AdBlock, false);
        assert!(contents.contains("chave=valor"));
        assert!(!enabled(&contents, Toggle::AdBlock));
    }

    #[test]
    fn existing_adblock_choice_is_still_read() {
        assert!(!enabled(
            "[navegacao]\nbloquear-anuncios=false\n",
            Toggle::AdBlock
        ));
    }

    #[test]
    fn invalid_files_fall_back_to_defaults() {
        assert!(enabled("isto não é ini", Toggle::AdBlock));
    }
}
