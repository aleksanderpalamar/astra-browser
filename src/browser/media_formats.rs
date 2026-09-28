pub struct MediaFormat {
    pub label: &'static str,
    pub mime_type: &'static str,
}

pub const ESSENTIAL_FORMATS: [MediaFormat; 4] = [
    MediaFormat {
        label: "vídeo VP9",
        mime_type: r#"video/webm; codecs="vp9""#,
    },
    MediaFormat {
        label: "vídeo H.264",
        mime_type: r#"video/mp4; codecs="avc1.4d401f""#,
    },
    MediaFormat {
        label: "áudio Opus",
        mime_type: r#"audio/webm; codecs="opus""#,
    },
    MediaFormat {
        label: "áudio AAC",
        mime_type: r#"audio/mp4; codecs="mp4a.40.2""#,
    },
];

const INSTALL_COMMAND: &str = "sudo pacman -S --needed gst-plugins-good gst-plugins-bad gst-libav";

pub fn unsupported_formats_warning(unsupported_mime_types: &str) -> Option<String> {
    let labels: Vec<&str> = unsupported_mime_types
        .lines()
        .filter_map(label_of)
        .collect();
    if labels.is_empty() {
        return None;
    }
    Some(format!(
        "Aviso: o WebKit não consegue reproduzir {}. Vídeos de sites como o YouTube podem falhar.\n\
         Instale os plugins de mídia do GStreamer: {INSTALL_COMMAND}",
        labels.join(", ")
    ))
}

fn label_of(mime_type: &str) -> Option<&'static str> {
    ESSENTIAL_FORMATS
        .iter()
        .find(|format| format.mime_type == mime_type)
        .map(|format| format.label)
}

#[cfg(test)]
mod tests {
    use super::{ESSENTIAL_FORMATS, unsupported_formats_warning};

    const OPUS: &str = r#"audio/webm; codecs="opus""#;
    const AAC: &str = r#"audio/mp4; codecs="mp4a.40.2""#;

    #[test]
    fn no_warning_when_every_format_is_supported() {
        assert_eq!(unsupported_formats_warning(""), None);
    }

    #[test]
    fn warns_about_missing_youtube_audio_codecs() {
        let warning = unsupported_formats_warning(&format!("{OPUS}\n{AAC}")).unwrap_or_default();
        assert!(warning.contains("áudio Opus, áudio AAC"));
        assert!(warning.contains("YouTube"));
        assert!(warning.contains("gst-plugins-bad gst-libav"));
    }

    #[test]
    fn ignores_unknown_mime_types() {
        assert_eq!(unsupported_formats_warning("video/x-unknown\n\n"), None);
    }

    #[test]
    fn every_essential_format_is_recognized() {
        for format in &ESSENTIAL_FORMATS {
            let warning = unsupported_formats_warning(format.mime_type).unwrap_or_default();
            assert!(warning.contains(format.label), "{}", format.label);
        }
    }
}
