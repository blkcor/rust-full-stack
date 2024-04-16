use dioxus::prelude::*;

use crate::models::ButtonType;

#[component]
pub fn Button(
    button_type: ButtonType,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx!(button {
        class: "text-slate-200 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded mt-4 md:mt-0 {button_type.to_string()}",
        onclick: move |event| onclick.call(event),
        div {{children}}
    })
}
