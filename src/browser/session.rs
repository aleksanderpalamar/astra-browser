use std::fs;
use std::path::{Path, PathBuf};

use webkit6::{CookiePersistentStorage, NetworkSession};

const COOKIES_FILE: &str = "cookies.sqlite";

pub fn persist_cookies(session: &NetworkSession) {
    let Some(directory) = session
        .website_data_manager()
        .and_then(|manager| manager.base_data_directory())
    else {
        eprintln!("Sessão sem diretório de dados; os cookies não serão salvos");
        return;
    };
    let Some(cookies) = session.cookie_manager() else {
        eprintln!("Gerenciador de cookies indisponível; os cookies não serão salvos");
        return;
    };
    if let Err(error) = fs::create_dir_all(directory.as_str()) {
        eprintln!("Não foi possível criar {directory}: {error}");
        return;
    }
    let path = cookies_path(Path::new(directory.as_str()));
    match path.to_str() {
        Some(path) => cookies.set_persistent_storage(path, CookiePersistentStorage::Sqlite),
        None => eprintln!("Caminho de cookies inválido: {}", path.display()),
    }
}

fn cookies_path(data_directory: &Path) -> PathBuf {
    data_directory.join(COOKIES_FILE)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::cookies_path;

    #[test]
    fn cookies_live_in_the_session_data_directory() {
        assert_eq!(
            cookies_path(Path::new("/home/u/.local/share/astra-browser")),
            Path::new("/home/u/.local/share/astra-browser/cookies.sqlite")
        );
    }
}
