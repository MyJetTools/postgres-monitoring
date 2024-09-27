pub const STORAGE_LEVEL_KEY: &str = "level";

pub fn get() -> SelectedLevel {
    let level = dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(STORAGE_LEVEL_KEY)
        .unwrap_or_default();

    SelectedLevel::from_str(&level)
}

pub fn set(level: SelectedLevel) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(STORAGE_LEVEL_KEY, level.as_str());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedLevel {
    All,
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}

impl SelectedLevel {
    pub const LEVELS: [(&str, &str); 6] = [
        (LOG_LEVEL_ALL_ID, "All"),
        (LOG_LEVEL_WARNING_ID, "Warning"),
        (LOG_LEVEL_ERROR_ID, "Error"),
        (LOG_LEVEL_FATAL_ERROR_ID, "FatalError"),
        (LOG_LEVEL_INFO_ID, "Info"),
        (LOG_LEVEL_DEBUG_ID, "Debug"),
    ];

    pub fn as_str(&self) -> &str {
        match self {
            SelectedLevel::All => LOG_LEVEL_ALL_ID,
            SelectedLevel::Info => LOG_LEVEL_INFO_ID,
            SelectedLevel::Warning => LOG_LEVEL_WARNING_ID,
            SelectedLevel::Error => LOG_LEVEL_ERROR_ID,
            SelectedLevel::FatalError => LOG_LEVEL_FATAL_ERROR_ID,
            SelectedLevel::Debug => LOG_LEVEL_DEBUG_ID,
        }
    }
    pub fn from_str(src: &str) -> Self {
        if rust_extensions::str_utils::compare_strings_case_insensitive(LOG_LEVEL_ERROR_ID, src) {
            return SelectedLevel::Error;
        }

        if rust_extensions::str_utils::compare_strings_case_insensitive(
            LOG_LEVEL_FATAL_ERROR_ID,
            src,
        ) {
            return SelectedLevel::FatalError;
        }

        if rust_extensions::str_utils::compare_strings_case_insensitive("fatalerror", src) {
            return SelectedLevel::FatalError;
        }

        if rust_extensions::str_utils::compare_strings_case_insensitive(LOG_LEVEL_WARNING_ID, src) {
            return SelectedLevel::Warning;
        }

        if rust_extensions::str_utils::compare_strings_case_insensitive(LOG_LEVEL_INFO_ID, src) {
            return SelectedLevel::Info;
        }

        if rust_extensions::str_utils::compare_strings_case_insensitive(LOG_LEVEL_DEBUG_ID, src) {
            return SelectedLevel::Debug;
        }

        SelectedLevel::All
    }
}

pub const LOG_LEVEL_ALL_ID: &str = "all";
pub const LOG_LEVEL_WARNING_ID: &str = "warning";
pub const LOG_LEVEL_INFO_ID: &str = "info";
pub const LOG_LEVEL_DEBUG_ID: &str = "debug";
pub const LOG_LEVEL_ERROR_ID: &str = "error";
pub const LOG_LEVEL_FATAL_ERROR_ID: &str = "fatal";
