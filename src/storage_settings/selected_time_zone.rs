const UTC_0_VALUE: &str = "0";
const LOCAL_VALUE: &str = "l";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTimeZone {
    UtcZero,
    LocalTime,
}

impl SelectedTimeZone {
    pub fn from_str(src: &str) -> Self {
        match src {
            UTC_0_VALUE => Self::UtcZero,
            LOCAL_VALUE => Self::LocalTime,
            _ => Self::UtcZero,
        }
    }
}

pub const SELECTED_TIME_ZONE: &str = "selected-timezone";

pub fn get() -> SelectedTimeZone {
    let time_zone = dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(SELECTED_TIME_ZONE)
        .unwrap_or_default();

    SelectedTimeZone::from_str(&time_zone)
}

pub fn set(time_zone: SelectedTimeZone) {
    match time_zone {
        SelectedTimeZone::UtcZero => {
            dioxus_utils::js::GlobalAppSettings::get_local_storage().delete(SELECTED_TIME_ZONE);
        }
        SelectedTimeZone::LocalTime => {
            dioxus_utils::js::GlobalAppSettings::get_local_storage()
                .set(SELECTED_TIME_ZONE, LOCAL_VALUE);
        }
    }
}
