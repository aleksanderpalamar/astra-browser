use webkit6::NetworkSession;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowsingMode {
    Normal,
    Private,
}

impl BrowsingMode {
    pub fn network_session(self) -> Option<NetworkSession> {
        match self {
            Self::Normal => NetworkSession::default(),
            Self::Private => Some(NetworkSession::new_ephemeral()),
        }
    }

    pub fn records_history(self) -> bool {
        self == Self::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::BrowsingMode;

    #[test]
    fn only_normal_browsing_records_history() {
        assert!(BrowsingMode::Normal.records_history());
        assert!(!BrowsingMode::Private.records_history());
    }
}
