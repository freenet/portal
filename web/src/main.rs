#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

const _BULMA_STYLE: &str = manganis::mg!(file("assets/bulma.min.css"));

const _STYLE: &str = manganis::mg!(file("assets/style.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        section {
            class: "section",
            div {
                class: "container",
                h1 { class: "title", "Welcome to Freenet" }
                h2 { class: "subtitle", "Declare your digital independence" }
                p { "The centralization of the internet poses a fundamental threat to individual
                freedom. In 2024, a few corporations control most internet services and infrastructure.
                These corporations wield immense power over most of us with little accountability,
                enabling them to censor content, exploit our data, and exclude users from services
                they depend on —all with profound implications for democracy. We need a solution
                urgently." }

                p { "Introducing Freenet — a decentralized replacement for the world wide web.
                Acting as a global, shared, decentralized computing platform, Freenet can either be
                accessed via a standard web browser or integrated into third-party applications." }

                p { "Freenet is not merely a tool for developers; it offers a pathway for anyone
                seeking greater control and freedom in their digital interactions. You can build
                or use decentralized services for messaging, social media, email, and e-commerce.
                 These applications are designed for scalability and interoperability, secured
                 through modern cryptographic techniques." }
            }
        }
    }
}
