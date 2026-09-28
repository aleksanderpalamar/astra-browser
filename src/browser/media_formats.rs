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
const HARDWARE_DECODING_COMMAND: &str = "sudo pacman -S --needed gst-plugin-va";
const HARDWARE_DECODING: &str = "hardware";

pub const HARDWARE_DECODING_PROBE: &str = r#"
const types = ['video/mp4; codecs="avc1.640028"', 'video/webm; codecs="vp09.00.40.08"', 'video/mp4; codecs="av01.0.08M.08"'];
for (const contentType of types) {
  const video = { contentType, width: 1920, height: 1080, bitrate: 5000000, framerate: 30 };
  const info = await navigator.mediaCapabilities.decodingInfo({ type: "media-source", video });
  if (info.powerEfficient) return "hardware";
}
return "software";
"#;

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

pub fn hardware_decoding_warning(probe_result: &str) -> Option<String> {
    if probe_result.trim() == HARDWARE_DECODING {
        return None;
    }
    Some(format!(
        "Aviso: os vídeos estão sendo decodificados pela CPU, sem aceleração da GPU.\n\
         Para decodificar pela GPU (AMD/Intel via VA-API), instale: {HARDWARE_DECODING_COMMAND}"
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
    use super::{ESSENTIAL_FORMATS, hardware_decoding_warning, unsupported_formats_warning};

    const OPUS: &str = r#"audio/webm; codecs="opus""#;
    const AAC: &str = r#"audio/mp4; codecs="mp4a.40.2""#;

    #[test]
    fn no_warning_when_the_gpu_decodes_video() {
        assert_eq!(hardware_decoding_warning("hardware"), None);
    }

    #[test]
    fn suggests_the_va_plugin_when_decoding_on_the_cpu() {
        let warning = hardware_decoding_warning("software").unwrap_or_default();
        assert!(warning.contains("CPU"));
        assert!(warning.contains("gst-plugin-va"));
    }

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
