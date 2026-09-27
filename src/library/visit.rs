use crate::library::tsv;

const RECORDED_SCHEMES: [&str; 2] = ["http://", "https://"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Visit {
    pub visited_at: u64,
    pub uri: String,
    pub title: String,
}

impl Visit {
    pub fn encode(&self) -> String {
        tsv::encode(&[&self.visited_at.to_string(), &self.uri, &self.title])
    }

    pub fn decode(line: &str) -> Option<Self> {
        let [visited_at, uri, title] = tsv::decode(line)[..] else {
            return None;
        };
        let visited_at = visited_at.parse().ok()?;
        is_recordable(uri).then(|| Self {
            visited_at,
            uri: uri.to_owned(),
            title: title.to_owned(),
        })
    }

    pub fn matches(&self, lowercase_query: &str) -> bool {
        lowercase_query.is_empty()
            || self.title.to_lowercase().contains(lowercase_query)
            || self.uri.to_lowercase().contains(lowercase_query)
    }
}

pub fn is_recordable(uri: &str) -> bool {
    RECORDED_SCHEMES
        .iter()
        .any(|scheme| uri.starts_with(scheme))
}

#[cfg(test)]
mod tests {
    use super::{Visit, is_recordable};

    #[test]
    fn records_only_web_pages() {
        assert!(is_recordable("https://github.com/"));
        assert!(is_recordable("http://localhost:3000/"));
        assert!(!is_recordable("about:blank"));
        assert!(!is_recordable("file:///etc/hosts"));
    }

    #[test]
    fn decodes_what_was_encoded() {
        let visit = Visit {
            visited_at: 42,
            uri: "https://github.com/".to_owned(),
            title: "GitHub".to_owned(),
        };
        assert_eq!(Visit::decode(&visit.encode()), Some(visit));
    }

    #[test]
    fn rejects_malformed_lines() {
        assert_eq!(Visit::decode("x\thttps://a.com/\tA"), None);
        assert_eq!(Visit::decode("5\tabout:blank\tB"), None);
        assert_eq!(Visit::decode("lixo"), None);
    }
}
