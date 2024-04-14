#![allow(non_snake_case)]

use dioxus::prelude::*;
fn main() {
    //initialize the logger
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    log::info!("hello console");
    // Launch the web application using the App component as the root.
    launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        div {
            "Hello, DevBcn!"
        }
    }
}
