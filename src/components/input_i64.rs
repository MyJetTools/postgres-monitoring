use dioxus::prelude::*;

use super::InputValue;

#[component]
pub fn InputI64(caption: String, value: i64, on_input: EventHandler<InputValue<i64>>) -> Element {
    let mut has_error = use_signal(|| false);
    let validation_attr = if *has_error.read() { "is-invalid" } else { "" };

    let mut value_state = use_signal(|| value.to_string());

    rsx! {
        div { class: "edit-wrapper {validation_attr}",
            label { {caption} }

            input {
                class: "form-control {validation_attr}",
                value: value_state.read().as_str(),
                r#type: "number",

                oninput: move |e| {
                    let value = e.value();
                    let value = value.trim();
                    *value_state.write() = value.to_string();
                    if value == "" {
                        *has_error.write() = true;
                        on_input.call(InputValue::InvalidValue);
                        return;
                    }
                    let value: Option<i64> = value.parse().ok();
                    match value {
                        Some(value) => {
                            *has_error.write() = false;
                            on_input.call(InputValue::Value(value));
                        }
                        None => {
                            *has_error.write() = true;
                            on_input.call(InputValue::InvalidValue);
                        }
                    }
                }
            }
        }
    }
}
