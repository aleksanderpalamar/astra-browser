use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use url::{Host, Url};

const SEARCH_ENDPOINT: &str = "https://duckduckgo.com/?q=";
const EXPLICIT_SCHEMES: [&str; 5] = ["http", "https", "file", "about", "webkit"];
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
        .or_else(|| implicit_https_url(input))
        .unwrap_or_else(|| search_url(input));
    Some(uri)
}

fn explicit_url(input: &str) -> Option<String> {
    let url = Url::parse(input).ok()?;
    EXPLICIT_SCHEMES
        .contains(&url.scheme())
        .then(|| input.to_owned())
}

fn implicit_https_url(input: &str) -> Option<String> {
    if input.contains(char::is_whitespace) {
        return None;
    }
    let candidate = format!("https://{input}");
    let url = Url::parse(&candidate).ok()?;
    is_navigable_host(&url, input).then_some(candidate)
}

fn is_navigable_host(url: &Url, input: &str) -> bool {
    if !url.username().is_empty() || url.password().is_some() {
        return false;
    }
    match url.host() {
        Some(Host::Domain(domain)) => domain == "localhost" || has_top_level_domain(domain),
        Some(Host::Ipv4(_)) => url.host_str().is_some_and(|host| input.starts_with(host)),
        Some(Host::Ipv6(_)) => true,
        None => false,
    }
}

fn has_top_level_domain(domain: &str) -> bool {
    let Some((name, tld)) = domain.rsplit_once('.') else {
        return false;
    };
    let is_valid_tld = tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic());
    !name.is_empty() && (is_valid_tld || tld.starts_with("xn--"))
}

fn search_url(query: &str) -> String {
    format!(
        "{SEARCH_ENDPOINT}{}",
        utf8_percent_encode(query, QUERY_COMPONENT)
    )
}

#[cfg(test)]
mod tests {
    use super::resolve_address;

    fn resolve(input: &str) -> String {
        resolve_address(input).unwrap_or_default()
    }

    #[test]
    fn ignores_empty_input() {
        assert_eq!(resolve_address(""), None);
        assert_eq!(resolve_address("   "), None);
    }

    #[test]
    fn prefixes_bare_domain_with_https() {
        assert_eq!(resolve("github.com"), "https://github.com");
        assert_eq!(resolve("  github.com  "), "https://github.com");
    }

    #[test]
    fn keeps_path_and_query_of_bare_domain() {
        assert_eq!(
            resolve("github.com/rust-lang/rust?tab=readme"),
            "https://github.com/rust-lang/rust?tab=readme"
        );
    }

    #[test]
    fn keeps_explicit_http_and_https_urls_intact() {
        assert_eq!(resolve("https://github.com"), "https://github.com");
        assert_eq!(resolve("http://example.com"), "http://example.com");
        assert_eq!(resolve("HTTPS://GitHub.com/x"), "HTTPS://GitHub.com/x");
    }

    #[test]
    fn keeps_other_supported_schemes_intact() {
        assert_eq!(resolve("about:blank"), "about:blank");
        assert_eq!(resolve("file:///etc/hosts"), "file:///etc/hosts");
    }

    #[test]
    fn opens_webkit_diagnostic_pages() {
        assert_eq!(resolve("webkit://gpu"), "webkit://gpu");
    }

    #[test]
    fn accepts_localhost_and_ip_addresses() {
        assert_eq!(resolve("localhost:8080"), "https://localhost:8080");
        assert_eq!(resolve("192.168.0.1"), "https://192.168.0.1");
        assert_eq!(resolve("[::1]:3000"), "https://[::1]:3000");
    }

    #[test]
    fn searches_plain_text() {
        assert_eq!(
            resolve("Rust ownership"),
            "https://duckduckgo.com/?q=Rust%20ownership"
        );
        assert_eq!(resolve("rust"), "https://duckduckgo.com/?q=rust");
    }

    #[test]
    fn encodes_reserved_characters_in_search() {
        assert_eq!(
            resolve("C++ & Rust #1 = 100%"),
            "https://duckduckgo.com/?q=C%2B%2B%20%26%20Rust%20%231%20%3D%20100%25"
        );
    }

    #[test]
    fn encodes_unicode_in_search() {
        assert_eq!(resolve("ação"), "https://duckduckgo.com/?q=a%C3%A7%C3%A3o");
    }

    #[test]
    fn searches_ambiguous_inputs() {
        assert_eq!(resolve("3.14"), "https://duckduckgo.com/?q=3.14");
        assert_eq!(
            resolve("user@example.com"),
            "https://duckduckgo.com/?q=user%40example.com"
        );
        assert_eq!(
            resolve("github.com."),
            "https://duckduckgo.com/?q=github.com."
        );
    }

    #[test]
    fn searches_text_with_unknown_scheme() {
        assert_eq!(
            resolve("rust: ownership"),
            "https://duckduckgo.com/?q=rust%3A%20ownership"
        );
    }
}
