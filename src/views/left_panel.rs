use dioxus::prelude::*;

use crate::LocationState;

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut dashboard_active = "";

    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    match location_state_value {
        LocationState::Dashboard => {
            dashboard_active = CLASS_NAME;
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

        Link { class: "menu-item {dashboard_active}", to: "/", "Dashboard" }

        div { class: "server-info", {client_version} }
    }
}

fn format_hours_to_gc(value: u32) -> String {
    if value < 24 {
        format!("{}h", value)
    } else {
        let days = value / 24;
        let hours = value % 24;
        format!("{}d {}h", days, hours)
    }
}

#[cfg(test)]
mod tests {
    use super::format_hours_to_gc;

    #[test]
    fn test() {
        let result = format_hours_to_gc(23);

        assert_eq!(result, "23h");

        let result = format_hours_to_gc(24);

        assert_eq!(result, "1d 0h");

        let result = format_hours_to_gc(25);

        assert_eq!(result, "1d 1h");
    }
}
