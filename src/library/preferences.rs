use std::path::PathBuf;

use gtk::glib::{KeyFile, KeyFileFlags};

use crate::library::files;

const GROUP: &str = "navegacao";
const ADBLOCK: Toggle = Toggle {
    key: "bloquear-anuncios",
    default: true,
};
const LOW_MEMORY: Toggle = Toggle {
    key: "baixo-consumo-memoria",
    default: false,
};

#[derive(Clone, Copy)]
struct Toggle {
    key: &'static str,
    default: bool,
}

pub struct Preferences {
    path: PathBuf,
}

impl Preferences {
    pub fn open(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn adblock_enabled(&self) -> bool {
        is_enabled(&self.read(), ADBLOCK)
    }

    pub fn set_adblock_enabled(&self, enabled: bool) {
        self.write(ADBLOCK, enabled);
    }

    pub fn low_memory_enabled(&self) -> bool {
        is_enabled(&self.read(), LOW_MEMORY)
    }

    pub fn set_low_memory_enabled(&self, enabled: bool) {
        self.write(LOW_MEMORY, enabled);
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

    fn write(&self, toggle: Toggle, enabled: bool) {
        let contents = with_toggle(&self.read(), toggle, enabled);
        if let Err(error) = files::write_atomically(&self.path, &contents) {
            eprintln!(
                "Não foi possível salvar as preferências em {}: {error}",
                self.path.display()
            );
        }
    }
}

fn is_enabled(contents: &str, toggle: Toggle) -> bool {
    parse(contents)
        .boolean(GROUP, toggle.key)
        .unwrap_or(toggle.default)
}

fn with_toggle(contents: &str, toggle: Toggle, enabled: bool) -> String {
    let keys = parse(contents);
    keys.set_boolean(GROUP, toggle.key, enabled);
    keys.to_data().to_string()
}

fn parse(contents: &str) -> KeyFile {
    let keys = KeyFile::new();
    if let Err(error) = keys.load_from_data(contents, KeyFileFlags::KEEP_COMMENTS)
        && !contents.trim().is_empty()
    {
        eprintln!("Preferências inválidas; usando os valores padrão: {error}");
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::{ADBLOCK, LOW_MEMORY, is_enabled, with_toggle};

    #[test]
    fn adblock_is_enabled_by_default() {
        assert!(is_enabled("", ADBLOCK));
        assert!(is_enabled("[outro]\nchave=1\n", ADBLOCK));
    }

    #[test]
    fn remembers_disabled_adblock() {
        let contents = with_toggle("", ADBLOCK, false);
        assert!(!is_enabled(&contents, ADBLOCK));
        assert!(is_enabled(&with_toggle(&contents, ADBLOCK, true), ADBLOCK));
    }

    #[test]
    fn low_memory_is_disabled_by_default() {
        assert!(!is_enabled("", LOW_MEMORY));
    }

    #[test]
    fn toggles_are_stored_independently() {
        let contents = with_toggle("", LOW_MEMORY, true);
        let contents = with_toggle(&contents, ADBLOCK, false);
        assert!(is_enabled(&contents, LOW_MEMORY));
        assert!(!is_enabled(&contents, ADBLOCK));
    }

    #[test]
    fn keeps_unrelated_settings() {
        let contents = with_toggle("[outro]\nchave=valor\n", ADBLOCK, false);
        assert!(contents.contains("chave=valor"));
        assert!(!is_enabled(&contents, ADBLOCK));
    }

    #[test]
    fn invalid_files_fall_back_to_defaults() {
        assert!(is_enabled("isto não é ini", ADBLOCK));
        assert!(!is_enabled("isto não é ini", LOW_MEMORY));
    }
}
