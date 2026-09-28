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
        Host::Domain(domain) => domain_scope(domain),
        Host::Ipv4(address) => url
            .host_str()
            .is_some_and(|host| input.starts_with(host))
            .then(|| ipv4_scope(address)),
        Host::Ipv6(address) => Some(ipv6_scope(address)),
    }
}

fn domain_scope(domain: &str) -> Option<HostScope> {
    if is_local_name(domain) {
        return Some(HostScope::Local);
    }
    has_top_level_domain(domain).then_some(HostScope::Public)
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

fn has_top_level_domain(domain: &str) -> bool {
    let Some((name, tld)) = domain.rsplit_once('.') else {
        return false;
    };
    let is_valid_tld = tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic());
    !name.is_empty() && (is_valid_tld || tld.starts_with("xn--"))
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
    use super::{HostScope, is_local_name};

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
