use dioxus::prelude::*;

#[component]
pub fn InputBool(value: bool, on_change: EventHandler<bool>) -> Element {
    rsx! {

        input {
            class: "form-check-input",
            style: " box-shadow: 0 0 1px gray;  margin-left: 5px; cursor: pointer;",
            r#type: "checkbox",
            checked: value,

            oninput: move |e| {
                let value = e.checked();
                on_change.call(value);
            }
        }
    }
}
