use dioxus::prelude::*;

#[component]
pub fn InputString(
    caption: String,
    value: String,
    read_only: Option<bool>,
    on_input: EventHandler<String>,
    on_enter_pressed: Option<EventHandler<()>>,
) -> Element {
    let style = if read_only.unwrap_or(false) {
        "color: lightgray;font-weight: bold;"
    } else {
        ""
    };

    rsx! {
        div { class: "edit-wrapper",
            label { {caption} }

            input {
                class: "form-control",
                r#type: "text",
                style: "{style}",
                value,
                readonly: read_only,
                oninput: move |e| {
                    let value = e.value().trim().to_string();
                    on_input.call(value);
                },

                onkeyup: move |e| {
                    match e.key() {
                        Key::Enter => {
                            if let Some(on_enter_pressed) = on_enter_pressed.as_ref() {
                                on_enter_pressed.call(());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
