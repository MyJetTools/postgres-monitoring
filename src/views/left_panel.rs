use dioxus::prelude::*;

use crate::LocationState;

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut dashboard_active = "";
    let mut db_size_active = "";

    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    match location_state_value {
        LocationState::Dashboard => {
            dashboard_active = CLASS_NAME;
        }

        LocationState::DbSize => {
            db_size_active = CLASS_NAME;
        }
    }

    let client_version = env!("CARGO_PKG_VERSION");
    let client_version = rsx! {

        div { "Client ver: {client_version}" }
    };

    rsx! {

        div {
            h1 { style: "color:white; padding:5px; text-align:center", "Postgres" }
        }

        div { style: "padding: 5px" }

        Link { class: "menu-item {dashboard_active}", to: crate::Route::Home {}, "Dashboard" }

        Link { class: "menu-item {db_size_active}", to: crate::Route::DbSize {}, "Db Size" }

        div { class: "server-info", {client_version} }
    }
}
