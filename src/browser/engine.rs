use webkit6::{HardwareAccelerationPolicy, Settings, UserContentManager};

#[derive(Clone)]
pub struct WebEngine {
    pub settings: Settings,
    pub content: UserContentManager,
}

impl WebEngine {
    pub fn new(content: UserContentManager) -> Self {
        let settings = Settings::new();
        settings.set_enable_developer_extras(true);
        settings.set_enable_media_stream(true);
        settings.set_enable_webrtc(true);
        Self { settings, content }
    }

    pub fn set_hardware_acceleration(&self, enabled: bool) {
        self.settings
            .set_hardware_acceleration_policy(acceleration_policy(enabled));
    }
}

fn acceleration_policy(enabled: bool) -> HardwareAccelerationPolicy {
    if enabled {
        HardwareAccelerationPolicy::Always
    } else {
        HardwareAccelerationPolicy::Never
    }
}

#[cfg(test)]
mod tests {
    use webkit6::HardwareAccelerationPolicy;

    use super::acceleration_policy;

    #[test]
    fn maps_the_toggle_to_webkit_policies() {
        assert_eq!(
            acceleration_policy(true),
            HardwareAccelerationPolicy::Always
        );
        assert_eq!(
            acceleration_policy(false),
            HardwareAccelerationPolicy::Never
        );
    }
}
