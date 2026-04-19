use dioxus::prelude::*;

mod platform;
mod api;
mod assets_wiki;

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "min-h-screen bg-bg-base text-[color:var(--fg-base)] flex items-center justify-center",
            h1 { class: "text-3xl font-semibold text-star-5", "鸣潮抽卡记录 — Dioxus Desktop" }
        }
    }
}
