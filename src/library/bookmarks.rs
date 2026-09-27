use crate::library::tsv;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bookmark {
    pub uri: String,
    pub title: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookmarkState {
    Bookmarked,
    NotBookmarked,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Bookmarks {
    entries: Vec<Bookmark>,
}

impl Bookmarks {
    pub fn parse(contents: &str) -> Self {
        let mut bookmarks = Self::default();
        for bookmark in contents.lines().filter_map(parse_line) {
            if bookmarks.state_of(&bookmark.uri) == BookmarkState::NotBookmarked {
                bookmarks.entries.push(bookmark);
            }
        }
        bookmarks
    }

    pub fn serialize(&self) -> String {
        self.entries
            .iter()
            .map(|bookmark| tsv::encode(&[&bookmark.uri, &bookmark.title]) + "\n")
            .collect()
    }

    pub fn entries(&self) -> &[Bookmark] {
        &self.entries
    }

    pub fn state_of(&self, uri: &str) -> BookmarkState {
        if self.entries.iter().any(|bookmark| bookmark.uri == uri) {
            BookmarkState::Bookmarked
        } else {
            BookmarkState::NotBookmarked
        }
    }

    pub fn toggle(&mut self, uri: &str, title: &str) -> BookmarkState {
        if self.state_of(uri) == BookmarkState::Bookmarked {
            self.remove(uri);
            return BookmarkState::NotBookmarked;
        }
        let title = match title.trim() {
            "" => uri,
            trimmed => trimmed,
        };
        let bookmark = Bookmark {
            uri: uri.to_owned(),
            title: title.to_owned(),
        };
        self.entries.insert(0, bookmark);
        BookmarkState::Bookmarked
    }

    pub fn remove(&mut self, uri: &str) {
        self.entries.retain(|bookmark| bookmark.uri != uri);
    }
}

fn parse_line(line: &str) -> Option<Bookmark> {
    let [uri, title] = tsv::decode(line)[..] else {
        return None;
    };
    if uri.is_empty() {
        return None;
    }
    Some(Bookmark {
        uri: uri.to_owned(),
        title: title.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::{BookmarkState, Bookmarks};

    const GITHUB: &str = "https://github.com/";

    #[test]
    fn toggling_adds_then_removes() {
        let mut bookmarks = Bookmarks::default();
        assert_eq!(
            bookmarks.toggle(GITHUB, "GitHub"),
            BookmarkState::Bookmarked
        );
        assert_eq!(bookmarks.state_of(GITHUB), BookmarkState::Bookmarked);
        assert_eq!(
            bookmarks.toggle(GITHUB, "GitHub"),
            BookmarkState::NotBookmarked
        );
        assert_eq!(bookmarks.state_of(GITHUB), BookmarkState::NotBookmarked);
    }

    #[test]
    fn newest_bookmark_comes_first() {
        let mut bookmarks = Bookmarks::default();
        bookmarks.toggle("https://a.com/", "A");
        bookmarks.toggle("https://b.com/", "B");
        let titles: Vec<_> = bookmarks
            .entries()
            .iter()
            .map(|b| b.title.as_str())
            .collect();
        assert_eq!(titles, ["B", "A"]);
    }

    #[test]
    fn empty_title_falls_back_to_uri() {
        let mut bookmarks = Bookmarks::default();
        bookmarks.toggle(GITHUB, "  ");
        assert_eq!(bookmarks.entries()[0].title, GITHUB);
    }

    #[test]
    fn survives_serialization_round_trip() {
        let mut bookmarks = Bookmarks::default();
        bookmarks.toggle(GITHUB, "GitHub\tcom tab");
        bookmarks.toggle("https://duckduckgo.com/", "DuckDuckGo");
        let restored = Bookmarks::parse(&bookmarks.serialize());
        assert_eq!(restored.entries().len(), 2);
        assert_eq!(restored.entries()[1].title, "GitHub com tab");
    }

    #[test]
    fn parsing_skips_malformed_and_duplicated_lines() {
        let contents = format!("{GITHUB}\tGitHub\nlixo\n\tsem uri\n{GITHUB}\tDe novo\n");
        let bookmarks = Bookmarks::parse(&contents);
        assert_eq!(bookmarks.entries().len(), 1);
        assert_eq!(bookmarks.entries()[0].title, "GitHub");
    }

    #[test]
    fn removing_unknown_uri_is_harmless() {
        let mut bookmarks = Bookmarks::default();
        bookmarks.toggle(GITHUB, "GitHub");
        bookmarks.remove("https://outro.com/");
        assert_eq!(bookmarks.entries().len(), 1);
    }
}
