use std::time::{Duration, Instant};

use webkit6::WebProcessTerminationReason;

const FAILURE_PAGE_GRACE: Duration = Duration::from_secs(3);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovery {
    Reload,
    ShowFailure,
    GiveUp,
}

#[derive(Debug, Default)]
pub struct CrashRecovery {
    retried_uri: Option<String>,
    failure_shown_at: Option<Instant>,
}

impl CrashRecovery {
    pub fn decide(
        &mut self,
        reason: WebProcessTerminationReason,
        uri: &str,
        now: Instant,
    ) -> Recovery {
        if self.failure_page_just_shown(now) {
            return Recovery::GiveUp;
        }
        if reason == WebProcessTerminationReason::Crashed && !self.already_retried(uri) {
            self.retried_uri = Some(uri.to_owned());
            return Recovery::Reload;
        }
        self.failure_shown_at = Some(now);
        Recovery::ShowFailure
    }

    fn already_retried(&self, uri: &str) -> bool {
        self.retried_uri.as_deref() == Some(uri)
    }

    fn failure_page_just_shown(&self, now: Instant) -> bool {
        self.failure_shown_at
            .is_some_and(|shown_at| now.duration_since(shown_at) < FAILURE_PAGE_GRACE)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use webkit6::WebProcessTerminationReason::{Crashed, ExceededMemoryLimit, TerminatedByApi};

    use super::{CrashRecovery, FAILURE_PAGE_GRACE, Recovery};

    const PAGE: &str = "https://duckduckgo.com/";
    const OTHER_PAGE: &str = "https://github.com/";

    #[test]
    fn first_crash_reloads_the_page() {
        let mut recovery = CrashRecovery::default();
        assert_eq!(
            recovery.decide(Crashed, PAGE, Instant::now()),
            Recovery::Reload
        );
    }

    #[test]
    fn second_crash_of_the_same_page_shows_the_failure() {
        let mut recovery = CrashRecovery::default();
        let start = Instant::now();
        recovery.decide(Crashed, PAGE, start);
        assert_eq!(
            recovery.decide(Crashed, PAGE, start + Duration::from_secs(1)),
            Recovery::ShowFailure
        );
    }

    #[test]
    fn crash_of_another_page_gets_its_own_reload() {
        let mut recovery = CrashRecovery::default();
        let start = Instant::now();
        recovery.decide(Crashed, PAGE, start);
        assert_eq!(
            recovery.decide(Crashed, OTHER_PAGE, start + Duration::from_secs(1)),
            Recovery::Reload
        );
    }

    #[test]
    fn memory_limit_and_api_termination_never_reload() {
        let mut recovery = CrashRecovery::default();
        let start = Instant::now();
        assert_eq!(
            recovery.decide(ExceededMemoryLimit, PAGE, start),
            Recovery::ShowFailure
        );
        let mut recovery = CrashRecovery::default();
        assert_eq!(
            recovery.decide(TerminatedByApi, PAGE, start),
            Recovery::ShowFailure
        );
    }

    #[test]
    fn crash_right_after_the_failure_page_gives_up() {
        let mut recovery = CrashRecovery::default();
        let start = Instant::now();
        recovery.decide(ExceededMemoryLimit, PAGE, start);
        assert_eq!(
            recovery.decide(Crashed, OTHER_PAGE, start + Duration::from_millis(100)),
            Recovery::GiveUp
        );
    }

    #[test]
    fn crash_after_the_grace_period_shows_the_failure_again() {
        let mut recovery = CrashRecovery::default();
        let start = Instant::now();
        recovery.decide(Crashed, PAGE, start);
        recovery.decide(Crashed, PAGE, start);
        assert_eq!(
            recovery.decide(Crashed, PAGE, start + FAILURE_PAGE_GRACE),
            Recovery::ShowFailure
        );
    }
}
