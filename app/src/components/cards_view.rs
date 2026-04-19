use dioxus::prelude::*;
use wuwa_gacha_history::EnrichedPull;

#[derive(Props, Clone, PartialEq)]
pub struct CardsViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn CardsView(props: CardsViewProps) -> Element {
    let _ = &props.pulls;
    rsx! { div { class: "text-fg-dim", "卡片式视图（Task 21）" } }
}
