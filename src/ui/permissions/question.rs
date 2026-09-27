use url::Url;

const UNKNOWN_SITE: &str = "Este site";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureDevices {
    Microphone,
    Camera,
    MicrophoneAndCamera,
}

impl CaptureDevices {
    pub fn requested(audio: bool, video: bool) -> Option<Self> {
        match (audio, video) {
            (true, false) => Some(Self::Microphone),
            (false, true) => Some(Self::Camera),
            (true, true) => Some(Self::MicrophoneAndCamera),
            (false, false) => None,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Microphone => "o seu microfone",
            Self::Camera => "a sua câmera",
            Self::MicrophoneAndCamera => "o seu microfone e a sua câmera",
        }
    }
}

pub fn permission_question(page_uri: Option<&str>, devices: CaptureDevices) -> String {
    format!(
        "{} quer usar {}",
        site_name(page_uri),
        devices.description()
    )
}

fn site_name(page_uri: Option<&str>) -> String {
    page_uri
        .and_then(|uri| Url::parse(uri).ok())
        .and_then(|url| url.host_str().map(str::to_owned))
        .unwrap_or_else(|| UNKNOWN_SITE.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{CaptureDevices, permission_question};

    #[test]
    fn identifies_requested_devices() {
        assert_eq!(
            CaptureDevices::requested(true, false),
            Some(CaptureDevices::Microphone)
        );
        assert_eq!(
            CaptureDevices::requested(false, true),
            Some(CaptureDevices::Camera)
        );
        assert_eq!(
            CaptureDevices::requested(true, true),
            Some(CaptureDevices::MicrophoneAndCamera)
        );
        assert_eq!(CaptureDevices::requested(false, false), None);
    }

    #[test]
    fn names_the_requesting_site() {
        assert_eq!(
            permission_question(
                Some("https://chatgpt.com/c/123"),
                CaptureDevices::Microphone
            ),
            "chatgpt.com quer usar o seu microfone"
        );
        assert_eq!(
            permission_question(
                Some("https://meet.example.org/"),
                CaptureDevices::MicrophoneAndCamera
            ),
            "meet.example.org quer usar o seu microfone e a sua câmera"
        );
    }

    #[test]
    fn falls_back_when_the_site_is_unknown() {
        assert_eq!(
            permission_question(None, CaptureDevices::Camera),
            "Este site quer usar a sua câmera"
        );
        assert_eq!(
            permission_question(Some("about:blank"), CaptureDevices::Microphone),
            "Este site quer usar o seu microfone"
        );
    }
}
