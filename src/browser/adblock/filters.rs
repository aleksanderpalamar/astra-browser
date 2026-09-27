pub const FILTER_ID: &str = "easylist";
pub const FILTER_URL: &str =
    "https://easylist-downloads.adblockplus.org/easylist_min_content_blocker.json";
pub const TIMESTAMP_FILE: &str = "easylist.updated";
pub const EXTRA_FILTER_ID: &str = "astra-extra";
pub const EXTRA_RULES: &str = include_str!("extra_rules.json");

const UPDATE_INTERVAL_SECS: u64 = 7 * 24 * 60 * 60;

pub fn needs_update(last_update: Option<u64>, now: u64) -> bool {
    last_update.is_none_or(|updated_at| now.saturating_sub(updated_at) >= UPDATE_INTERVAL_SECS)
}

pub fn parse_timestamp(contents: &str) -> Option<u64> {
    contents.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{UPDATE_INTERVAL_SECS, needs_update, parse_timestamp};

    const NOW: u64 = 1_800_000_000;

    #[test]
    fn missing_list_must_be_downloaded() {
        assert!(needs_update(None, NOW));
    }

    #[test]
    fn recent_list_is_kept() {
        assert!(!needs_update(Some(NOW - 60), NOW));
        assert!(!needs_update(Some(NOW - UPDATE_INTERVAL_SECS + 1), NOW));
    }

    #[test]
    fn week_old_list_is_refreshed() {
        assert!(needs_update(Some(NOW - UPDATE_INTERVAL_SECS), NOW));
    }

    #[test]
    fn clock_going_backwards_keeps_the_list() {
        assert!(!needs_update(Some(NOW + 3600), NOW));
    }

    #[test]
    fn extra_rules_are_valid_content_blocker_rules() {
        let rules: Vec<serde_json::Value> =
            serde_json::from_str(super::EXTRA_RULES).unwrap_or_default();
        assert!(!rules.is_empty());
        for rule in &rules {
            assert!(rule["trigger"]["url-filter"].is_string());
            assert_eq!(rule["action"]["type"], "css-display-none");
            assert!(rule["action"]["selector"].is_string());
        }
    }

    #[test]
    fn extra_rules_cover_current_youtube_ad_slots() {
        assert!(super::EXTRA_RULES.contains("ytd-ad-slot-renderer"));
        assert!(super::EXTRA_RULES.contains("#player-ads"));
    }

    #[test]
    fn parses_stored_timestamps() {
        assert_eq!(parse_timestamp(" 1800000000\n"), Some(NOW));
        assert_eq!(parse_timestamp(""), None);
        assert_eq!(parse_timestamp("ontem"), None);
    }
}
