pub const STORAGE_CTX_SEARCH_LINE_KEY: &str = "ctx-search";

pub fn get() -> bool {
    let result = dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(STORAGE_CTX_SEARCH_LINE_KEY)
        .unwrap_or_default();

    if result == "0" {
        return false;
    }

    true
}

pub fn set(ctx_search: bool) {
    let storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();
    if ctx_search {
        storage.delete(STORAGE_CTX_SEARCH_LINE_KEY);
    } else {
        dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .set(STORAGE_CTX_SEARCH_LINE_KEY, "0");
    }
}
