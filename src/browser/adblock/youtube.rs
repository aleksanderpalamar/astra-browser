use webkit6::{UserContentInjectedFrames, UserScript, UserScriptInjectionTime};

const SOURCE: &str = include_str!("youtube.js");
const PAGES: [&str; 3] = [
    "https://youtube.com/*",
    "https://*.youtube.com/*",
    "https://*.youtube-nocookie.com/*",
];

pub fn player_ads_script() -> UserScript {
    UserScript::new(
        SOURCE,
        UserContentInjectedFrames::AllFrames,
        UserScriptInjectionTime::Start,
        &PAGES,
        &[],
    )
}
