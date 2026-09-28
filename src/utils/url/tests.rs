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
fn local_servers_use_http() {
    assert_eq!(resolve("localhost:3000"), "http://localhost:3000");
    assert_eq!(resolve("localhost"), "http://localhost");
    assert_eq!(
        resolve("app.localhost:5173/login"),
        "http://app.localhost:5173/login"
    );
    assert_eq!(resolve("127.0.0.1:8000"), "http://127.0.0.1:8000");
    assert_eq!(resolve("0.0.0.0:8000"), "http://0.0.0.0:8000");
    assert_eq!(resolve("[::1]:3000"), "http://[::1]:3000");
}

#[test]
fn private_network_addresses_use_http() {
    assert_eq!(resolve("192.168.0.1"), "http://192.168.0.1");
    assert_eq!(resolve("10.0.0.5:8080"), "http://10.0.0.5:8080");
    assert_eq!(resolve("172.16.4.2"), "http://172.16.4.2");
    assert_eq!(resolve("169.254.1.1"), "http://169.254.1.1");
    assert_eq!(resolve("[fd00::1]:8080"), "http://[fd00::1]:8080");
}

#[test]
fn local_network_names_use_http() {
    assert_eq!(resolve("nas.local"), "http://nas.local");
    assert_eq!(resolve("box.lan:8080"), "http://box.lan:8080");
    assert_eq!(resolve("api.internal/health"), "http://api.internal/health");
    assert_eq!(resolve("router.home.arpa"), "http://router.home.arpa");
    assert_eq!(resolve("blog.test"), "http://blog.test");
}

#[test]
fn public_addresses_keep_https() {
    assert_eq!(resolve("8.8.8.8"), "https://8.8.8.8");
    assert_eq!(resolve("172.32.0.1"), "https://172.32.0.1");
    assert_eq!(resolve("[2001:db8::1]"), "https://[2001:db8::1]");
    assert_eq!(resolve("localhost.com"), "https://localhost.com");
}

#[test]
fn explicit_scheme_wins_over_local_default() {
    assert_eq!(resolve("https://localhost:3000"), "https://localhost:3000");
    assert_eq!(resolve("http://github.com"), "http://github.com");
}

#[test]
fn bare_local_suffixes_are_searched() {
    assert_eq!(resolve("local"), "https://duckduckgo.com/?q=local");
    assert_eq!(resolve("test"), "https://duckduckgo.com/?q=test");
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
