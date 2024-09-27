pub const STORAGE_SEARCH_LINE_KEY: &str = "search-line";

pub fn get() -> String {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(STORAGE_SEARCH_LINE_KEY)
        .unwrap_or_default()
}

pub fn set(search_line: &str) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .set(STORAGE_SEARCH_LINE_KEY, search_line);
}
