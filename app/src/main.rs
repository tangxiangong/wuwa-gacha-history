use dioxus::prelude::*;

mod api;
mod assets_wiki;
mod components;
mod platform;
mod state;

use state::AppCtx;

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(AppCtx::init);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        components::Root {}
    }
}
