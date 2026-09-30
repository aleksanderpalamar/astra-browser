use webkit6::{UserContentInjectedFrames, UserScript, UserScriptInjectionTime};

const SOURCE: &str = include_str!("media_loop.js");

pub fn loop_guard_script() -> UserScript {
    UserScript::new(
        SOURCE,
        UserContentInjectedFrames::AllFrames,
        UserScriptInjectionTime::Start,
        &[],
        &[],
    )
}
