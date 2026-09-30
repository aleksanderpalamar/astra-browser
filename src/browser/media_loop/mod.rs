use webkit6::{UserContentInjectedFrames, UserScript, UserScriptInjectionTime};

const SOURCE: &str = concat!(
    "(() => {\n",
    include_str!("policy.js"),
    "\n",
    include_str!("guard.js"),
    "})();\n"
);

pub fn loop_guard_script() -> UserScript {
    UserScript::new(
        SOURCE,
        UserContentInjectedFrames::AllFrames,
        UserScriptInjectionTime::Start,
        &[],
        &[],
    )
}

#[cfg(test)]
mod tests {
    use webkit6::javascriptcore::Context;

    const POLICY: &str = include_str!("policy.js");
    const PLAYING_LOOP: &str = "loop: true, mediaSource: true, paused: false, seeking: false, \
         duration: 20, currentTime: 5, playbackRate: 1, seekableStart: 0";

    fn restart_time(overrides: &str) -> Option<f64> {
        let context = Context::new();
        context.evaluate(POLICY);
        let value = context.evaluate(&format!(
            "loopRestartTime({{ {PLAYING_LOOP}, {overrides} }})"
        ));
        assert!(context.exception().is_none(), "erro no policy.js");
        value
            .filter(|value| !value.is_null())
            .map(|value| value.to_double())
    }

    #[test]
    fn restarts_a_media_source_loop_just_before_the_end() {
        assert_eq!(restart_time("currentTime: 19.8"), Some(0.0));
    }

    #[test]
    fn keeps_playing_away_from_the_end() {
        assert_eq!(restart_time("currentTime: 10"), None);
    }

    #[test]
    fn ignores_videos_without_loop_or_media_source() {
        assert_eq!(restart_time("currentTime: 19.8, loop: false"), None);
        assert_eq!(restart_time("currentTime: 19.8, mediaSource: false"), None);
    }

    #[test]
    fn leaves_paused_or_seeking_videos_alone() {
        assert_eq!(restart_time("currentTime: 19.8, paused: true"), None);
        assert_eq!(restart_time("currentTime: 19.8, seeking: true"), None);
    }

    #[test]
    fn lets_clips_shorter_than_twice_the_margin_loop_natively() {
        assert_eq!(restart_time("duration: 0.3, currentTime: 0.1"), None);
        assert_eq!(restart_time("duration: 0.7, currentTime: 0.5"), None);
    }

    #[test]
    fn widens_the_margin_for_faster_playback() {
        assert_eq!(restart_time("currentTime: 19.5"), None);
        assert_eq!(
            restart_time("currentTime: 19.5, playbackRate: 2"),
            Some(0.0)
        );
    }

    #[test]
    fn restarts_at_the_first_seekable_time() {
        assert_eq!(
            restart_time("currentTime: 19.8, seekableStart: 5"),
            Some(5.0)
        );
    }

    #[test]
    fn ignores_live_streams() {
        assert_eq!(restart_time("duration: Infinity, currentTime: 19.8"), None);
    }
}
