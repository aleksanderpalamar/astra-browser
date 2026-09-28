use webkit6::{MemoryPressureSettings, WebContext};

use crate::browser::memory_mode::MemoryMode;

const MEMORY_LIMIT_MIB: u32 = 2048;
const NEVER_KILL: f64 = 0.0;

pub fn create(mode: MemoryMode) -> WebContext {
    set_global_cache_model(mode);
    WebContext::builder()
        .memory_pressure_settings(&memory_pressure())
        .build()
}

fn set_global_cache_model(mode: MemoryMode) {
    let Some(default) = WebContext::default() else {
        eprintln!(
            "Contexto padrão do WebKit indisponível; o modo de memória vale só para os caches"
        );
        return;
    };
    default.set_cache_model(mode.cache_model());
}

fn memory_pressure() -> MemoryPressureSettings {
    let mut settings = MemoryPressureSettings::new();
    settings.set_memory_limit(MEMORY_LIMIT_MIB);
    settings.set_kill_threshold(NEVER_KILL);
    settings
}

#[cfg(test)]
mod tests {
    use super::{MEMORY_LIMIT_MIB, NEVER_KILL, memory_pressure};

    const DEFAULT_CONSERVATIVE_THRESHOLD: f64 = 0.33;
    const DEFAULT_STRICT_THRESHOLD: f64 = 0.5;

    #[test]
    fn limits_each_web_process_without_killing_it() {
        let mut settings = memory_pressure();
        assert_eq!(settings.memory_limit(), MEMORY_LIMIT_MIB);
        assert_eq!(settings.kill_threshold(), NEVER_KILL);
    }

    #[test]
    fn keeps_the_default_cleanup_thresholds() {
        let mut settings = memory_pressure();
        assert_eq!(
            settings.conservative_threshold(),
            DEFAULT_CONSERVATIVE_THRESHOLD
        );
        assert_eq!(settings.strict_threshold(), DEFAULT_STRICT_THRESHOLD);
    }
}
