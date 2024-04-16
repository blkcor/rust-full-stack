#![allow(non_snake_case)]
mod components;
mod models;
use components::{FilmModal, Footer, Header};
use dioxus::prelude::*;

fn main() {
    //initialize the logger
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        main{
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header{},
            section{
                class: "md:container md:mx-auto md:py-8 flex-1"
            },
            FilmModal{
                on_create_or_update: |_|{},
                on_cancel: |_|{}
            },
            Footer{}
        }
    }
}
