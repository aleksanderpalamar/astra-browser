mod host;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use url::Url;

const SEARCH_ENDPOINT: &str = "https://duckduckgo.com/?q=";
const EXPLICIT_SCHEMES: [&str; 4] = ["http", "https", "file", "about"];
const QUERY_COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

pub fn resolve_address(input: &str) -> Option<String> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }
    let uri = explicit_url(input)
        .or_else(|| implicit_url(input))
        .unwrap_or_else(|| search_url(input));
    Some(uri)
}

fn explicit_url(input: &str) -> Option<String> {
    let url = Url::parse(input).ok()?;
    EXPLICIT_SCHEMES
        .contains(&url.scheme())
        .then(|| input.to_owned())
}

fn implicit_url(input: &str) -> Option<String> {
    if input.contains(char::is_whitespace) {
        return None;
    }
    let url = Url::parse(&format!("http://{input}")).ok()?;
    let scope = host::scope(&url, input)?;
    Some(format!("{}://{input}", scope.scheme()))
}

fn search_url(query: &str) -> String {
    format!(
        "{SEARCH_ENDPOINT}{}",
        utf8_percent_encode(query, QUERY_COMPONENT)
    )
}

#[cfg(test)]
mod tests;
