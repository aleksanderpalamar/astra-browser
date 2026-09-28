use gtk::glib;
use webkit6::WebProcessTerminationReason;

const TEMPLATE: &str = include_str!("failure.html");

pub fn failure_page(reason: WebProcessTerminationReason, uri: &str) -> String {
    TEMPLATE
        .replace("{reason}", explanation(reason))
        .replace("{uri}", &glib::markup_escape_text(uri))
}

fn explanation(reason: WebProcessTerminationReason) -> &'static str {
    match reason {
        WebProcessTerminationReason::Crashed => "O processo que exibia esta página travou.",
        WebProcessTerminationReason::ExceededMemoryLimit => {
            "Esta página usou memória demais e foi encerrada."
        }
        _ => "O processo que exibia esta página foi encerrado.",
    }
}

#[cfg(test)]
mod tests {
    use webkit6::WebProcessTerminationReason::{Crashed, ExceededMemoryLimit};

    use super::failure_page;

    #[test]
    fn links_the_reload_button_to_the_failed_page() {
        let page = failure_page(Crashed, "https://duckduckgo.com/");
        assert!(page.contains(r#"href="https://duckduckgo.com/""#));
        assert!(page.contains("travou"));
    }

    #[test]
    fn explains_the_memory_limit() {
        let page = failure_page(ExceededMemoryLimit, "https://duckduckgo.com/");
        assert!(page.contains("memória demais"));
    }

    #[test]
    fn escapes_the_address() {
        let page = failure_page(Crashed, r#"https://a.test/?q=<b>&x="1""#);
        assert!(page.contains("https://a.test/?q=&lt;b&gt;&amp;x=&quot;1&quot;"));
        assert!(!page.contains("<b>"));
        assert!(!page.contains("{uri}"));
    }
}
