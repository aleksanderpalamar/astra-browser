use std::net::{Ipv4Addr, Ipv6Addr};

use url::{Host, Url};

const LOCALHOST: &str = "localhost";
const LOCAL_SUFFIXES: [&str; 6] = [LOCALHOST, "local", "lan", "internal", "home.arpa", "test"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostScope {
    Local,
    Public,
}

impl HostScope {
    pub fn scheme(self) -> &'static str {
        match self {
            Self::Local => "http",
            Self::Public => "https",
        }
    }
}

pub fn scope(url: &Url, input: &str) -> Option<HostScope> {
    if !url.username().is_empty() || url.password().is_some() {
        return None;
    }
    match url.host()? {
        Host::Domain(domain) => domain_scope(domain, input),
        Host::Ipv4(address) => url
            .host_str()
            .is_some_and(|host| input.starts_with(host))
            .then(|| ipv4_scope(address)),
        Host::Ipv6(address) => Some(ipv6_scope(address)),
    }
}

fn domain_scope(domain: &str, input: &str) -> Option<HostScope> {
    if is_local_name(domain) {
        return Some(HostScope::Local);
    }
    if is_public_domain(domain) {
        return Some(HostScope::Public);
    }
    is_written_as_address(input).then_some(HostScope::Local)
}

fn is_written_as_address(input: &str) -> bool {
    input.contains([':', '/'])
}

fn is_local_name(domain: &str) -> bool {
    domain == LOCALHOST
        || LOCAL_SUFFIXES
            .iter()
            .any(|suffix| is_subdomain_of(domain, suffix))
}

fn is_subdomain_of(domain: &str, suffix: &str) -> bool {
    domain
        .strip_suffix(suffix)
        .and_then(|name| name.strip_suffix('.'))
        .is_some_and(|name| !name.is_empty())
}

fn is_public_domain(domain: &str) -> bool {
    domain.contains('.')
        && domain.split('.').all(|label| !label.is_empty())
        && psl::suffix(domain.as_bytes()).is_some_and(|suffix| suffix.is_known())
}

fn ipv4_scope(address: Ipv4Addr) -> HostScope {
    let is_local = address.is_loopback()
        || address.is_private()
        || address.is_link_local()
        || address.is_unspecified();
    if is_local {
        HostScope::Local
    } else {
        HostScope::Public
    }
}

fn ipv6_scope(address: Ipv6Addr) -> HostScope {
    if let Some(mapped) = address.to_ipv4_mapped() {
        return ipv4_scope(mapped);
    }
    let is_local = address.is_loopback()
        || address.is_unique_local()
        || address.is_unicast_link_local()
        || address.is_unspecified();
    if is_local {
        HostScope::Local
    } else {
        HostScope::Public
    }
}

#[cfg(test)]
mod tests {
    use super::{HostScope, is_local_name, is_public_domain, is_written_as_address};

    #[test]
    fn local_hosts_use_http_and_public_hosts_use_https() {
        assert_eq!(HostScope::Local.scheme(), "http");
        assert_eq!(HostScope::Public.scheme(), "https");
    }

    #[test]
    fn recognizes_local_network_names() {
        for name in [
            "localhost",
            "app.localhost",
            "nas.local",
            "box.lan",
            "api.internal",
            "router.home.arpa",
            "site.test",
        ] {
            assert!(is_local_name(name), "{name}");
        }
    }

    #[test]
    fn public_domains_need_a_known_suffix() {
        for domain in [
            "github.com",
            "exemplo.com.br",
            "gov.br",
            "pt.wikipedia.org",
            "xn--e1afmkfd.xn--p1ai",
        ] {
            assert!(is_public_domain(domain), "{domain}");
        }
        for domain in [
            "astra.ownership",
            "index.html",
            "intranet",
            "github.com.",
            ".com",
            "a..com",
        ] {
            assert!(!is_public_domain(domain), "{domain}");
        }
    }

    #[test]
    fn port_path_or_trailing_slash_mark_an_address() {
        assert!(is_written_as_address("intranet:8080"));
        assert!(is_written_as_address("intranet/"));
        assert!(is_written_as_address("nas/admin"));
        assert!(!is_written_as_address("intranet"));
    }

    #[test]
    fn bare_local_suffixes_are_not_hosts() {
        for name in [
            "local",
            "lan",
            "internal",
            "test",
            ".local",
            "notlocal",
            "github.com",
        ] {
            assert!(!is_local_name(name), "{name}");
        }
    }
}
