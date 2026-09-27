use serde_json::Value;

const EMPTY_DOMAIN_PATTERN: &str = r"^[^:]+:(//)?([^/]+\.)?[^\.][/:]?";
const REQUIRED_CONDITIONS: [&str; 2] = ["if-top-url", "if-domain"];
const EXCLUDING_CONDITIONS: [&str; 2] = ["unless-top-url", "unless-domain"];

pub fn sanitize(source: &[u8]) -> Result<Vec<u8>, serde_json::Error> {
    let mut rules: Vec<Value> = serde_json::from_slice(source)?;
    rules.retain_mut(repair);
    serde_json::to_vec(&rules)
}

fn repair(rule: &mut Value) -> bool {
    let Some(trigger) = rule.get_mut("trigger").and_then(Value::as_object_mut) else {
        return true;
    };
    for condition in EXCLUDING_CONDITIONS {
        if remove_empty_domains(trigger.get_mut(condition)) {
            trigger.remove(condition);
        }
    }
    !REQUIRED_CONDITIONS
        .into_iter()
        .any(|condition| remove_empty_domains(trigger.get_mut(condition)))
}

fn remove_empty_domains(patterns: Option<&mut Value>) -> bool {
    let Some(patterns) = patterns.and_then(Value::as_array_mut) else {
        return false;
    };
    patterns.retain(|pattern| pattern.as_str() != Some(EMPTY_DOMAIN_PATTERN));
    patterns.is_empty()
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{EMPTY_DOMAIN_PATTERN, sanitize};

    const GITLAB: &str = r"^[^:]+:(//)?([^/]+\.)?gitlab\.com[^\.][/:]?";

    fn run(rules: Value) -> Value {
        let source = serde_json::to_vec(&rules).unwrap_or_default();
        let output = sanitize(&source).unwrap_or_default();
        serde_json::from_slice(&output).unwrap_or(Value::Null)
    }

    #[test]
    fn removes_the_empty_domain_that_matches_every_site() {
        let rules = json!([{
            "trigger": {"url-filter": ".*", "if-top-url": [GITLAB, EMPTY_DOMAIN_PATTERN]},
            "action": {"type": "ignore-previous-rules"}
        }]);
        assert_eq!(run(rules)[0]["trigger"]["if-top-url"], json!([GITLAB]));
    }

    #[test]
    fn drops_rules_left_without_any_site() {
        let rules = json!([
            {"trigger": {"url-filter": ".*", "if-top-url": [EMPTY_DOMAIN_PATTERN]}, "action": {"type": "ignore-previous-rules"}},
            {"trigger": {"url-filter": ".*"}, "action": {"type": "css-display-none", "selector": "#ad"}}
        ]);
        let output = run(rules);
        assert_eq!(output.as_array().map(Vec::len), Some(1));
        assert_eq!(output[0]["action"]["selector"], "#ad");
    }

    #[test]
    fn empty_exclusions_make_the_rule_apply_everywhere() {
        let rules = json!([{
            "trigger": {"url-filter": ".*", "unless-top-url": [EMPTY_DOMAIN_PATTERN]},
            "action": {"type": "css-display-none", "selector": ".ad"}
        }]);
        let output = run(rules);
        assert_eq!(output[0]["trigger"], json!({"url-filter": ".*"}));
    }

    #[test]
    fn keeps_valid_rules_untouched() {
        let rules = json!([{
            "trigger": {"url-filter": "doubleclick", "if-domain": ["*example.com"]},
            "action": {"type": "block"}
        }]);
        assert_eq!(run(rules.clone()), rules);
    }

    #[test]
    fn rejects_invalid_lists() {
        assert!(sanitize(b"isto nao e json").is_err());
    }
}
