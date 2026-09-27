const SEPARATOR: char = '\t';

pub fn encode(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|field| field.replace(['\t', '\n', '\r'], " "))
        .collect::<Vec<_>>()
        .join("\t")
}

pub fn decode(line: &str) -> Vec<&str> {
    line.split(SEPARATOR).collect()
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn joins_fields_with_tabs() {
        assert_eq!(encode(&["a", "b", "c"]), "a\tb\tc");
    }

    #[test]
    fn replaces_separators_inside_fields() {
        assert_eq!(
            encode(&["tí\ttulo\ncom\rquebras", "x"]),
            "tí tulo com quebras\tx"
        );
    }

    #[test]
    fn decodes_what_was_encoded() {
        let line = encode(&["https://github.com", "GitHub · Build"]);
        assert_eq!(decode(&line), ["https://github.com", "GitHub · Build"]);
    }

    #[test]
    fn keeps_empty_fields() {
        assert_eq!(decode("a\t\tc"), ["a", "", "c"]);
    }
}
