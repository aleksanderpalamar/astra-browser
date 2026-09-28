use webkit6::CacheModel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryMode {
    Standard,
    Low,
}

impl MemoryMode {
    pub fn from_low_memory(enabled: bool) -> Self {
        if enabled { Self::Low } else { Self::Standard }
    }

    pub fn cache_model(self) -> CacheModel {
        match self {
            Self::Standard => CacheModel::WebBrowser,
            Self::Low => CacheModel::DocumentBrowser,
        }
    }

    pub fn keeps_page_cache(self) -> bool {
        self == Self::Standard
    }
}

#[cfg(test)]
mod tests {
    use webkit6::CacheModel;

    use super::MemoryMode;

    #[test]
    fn standard_mode_keeps_the_browser_caches() {
        let mode = MemoryMode::from_low_memory(false);
        assert_eq!(mode, MemoryMode::Standard);
        assert_eq!(mode.cache_model(), CacheModel::WebBrowser);
        assert!(mode.keeps_page_cache());
    }

    #[test]
    fn low_memory_mode_drops_the_page_cache() {
        let mode = MemoryMode::from_low_memory(true);
        assert_eq!(mode, MemoryMode::Low);
        assert_eq!(mode.cache_model(), CacheModel::DocumentBrowser);
        assert!(!mode.keeps_page_cache());
    }
}
