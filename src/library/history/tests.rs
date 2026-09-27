use super::{History, MAX_VISITS, Recording, Visit};

fn visit(at: u64, uri: &str, title: &str) -> Visit {
    Visit {
        visited_at: at,
        uri: uri.to_owned(),
        title: title.to_owned(),
    }
}

fn uris(visits: &[Visit]) -> Vec<&str> {
    visits.iter().map(|visit| visit.uri.as_str()).collect()
}

#[test]
fn ignores_immediate_repetition() {
    let mut history = History::default();
    assert_eq!(
        history.record(visit(1, "https://a.com/", "A")),
        Recording::Added
    );
    assert_eq!(
        history.record(visit(2, "https://a.com/", "A")),
        Recording::Ignored
    );
    assert_eq!(
        history.record(visit(3, "https://b.com/", "B")),
        Recording::Added
    );
    assert_eq!(
        history.record(visit(4, "https://a.com/", "A")),
        Recording::Added
    );
    assert_eq!(history.len(), 3);
}

#[test]
fn late_title_completes_the_last_visit() {
    let mut history = History::default();
    history.record(visit(1, "https://a.com/", ""));
    assert_eq!(
        history.record(visit(2, "https://a.com/", "A")),
        Recording::Retitled
    );
    assert_eq!(history.last(), Some(&visit(1, "https://a.com/", "A")));
    assert_eq!(
        history.record(visit(3, "https://a.com/", "Outro")),
        Recording::Ignored
    );
    assert_eq!(history.len(), 1);
}

#[test]
fn ignores_pages_that_are_not_web() {
    let mut history = History::default();
    assert_eq!(
        history.record(visit(1, "about:blank", "")),
        Recording::Ignored
    );
    assert_eq!(history.len(), 0);
}

#[test]
fn search_lists_newest_first_without_duplicates() {
    let mut history = History::default();
    for (at, uri) in [
        (1, "https://a.com/"),
        (2, "https://b.com/"),
        (3, "https://a.com/"),
    ] {
        history.record(visit(at, uri, ""));
    }
    assert_eq!(
        uris(&history.search("", 10)),
        ["https://a.com/", "https://b.com/"]
    );
}

#[test]
fn search_matches_title_or_uri_ignoring_case() {
    let mut history = History::default();
    history.record(visit(1, "https://github.com/", "GitHub"));
    history.record(visit(2, "https://duckduckgo.com/", "DuckDuckGo"));
    assert_eq!(uris(&history.search("git", 10)), ["https://github.com/"]);
    assert_eq!(
        uris(&history.search(" DUCKDUCK ", 10)),
        ["https://duckduckgo.com/"]
    );
    assert!(history.search("inexistente", 10).is_empty());
}

#[test]
fn search_respects_limit() {
    let mut history = History::default();
    for at in 0..5 {
        history.record(visit(at, &format!("https://{at}.com/"), ""));
    }
    assert_eq!(history.search("", 2).len(), 2);
}

#[test]
fn keeps_only_the_most_recent_visits() {
    let mut history = History::default();
    for at in 0..=MAX_VISITS as u64 {
        history.record(visit(at, &format!("https://{at}.com/"), ""));
    }
    assert_eq!(history.len(), MAX_VISITS);
    assert_eq!(
        uris(&history.search("", 1)),
        [format!("https://{MAX_VISITS}.com/")]
    );
    assert!(history.search("https://0.com/", 1).is_empty());
}

#[test]
fn survives_serialization_round_trip() {
    let mut history = History::default();
    history.record(visit(10, "https://github.com/", "GitHub\ttitle"));
    let restored = History::parse(&history.serialize());
    assert_eq!(
        restored.search("", 1),
        [visit(10, "https://github.com/", "GitHub title")]
    );
}

#[test]
fn parsing_skips_malformed_lines() {
    let contents = "x\thttps://a.com/\tA\n5\tabout:blank\tB\n7\thttps://c.com/\tC\nlixo\n";
    assert_eq!(
        uris(&History::parse(contents).search("", 10)),
        ["https://c.com/"]
    );
}

#[test]
fn clearing_forgets_every_visit() {
    let mut history = History::default();
    history.record(visit(1, "https://a.com/", "A"));
    history.clear();
    assert_eq!(history.len(), 0);
}
