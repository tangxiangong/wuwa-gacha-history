use dioxus::prelude::*;
use wuwa_gacha_history::EnrichedPull;

#[derive(Props, Clone, PartialEq)]
pub struct BarsViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn BarsView(props: BarsViewProps) -> Element {
    let _ = &props.pulls;
    rsx! { div { class: "text-fg-dim", "条形式视图（Task 20）" } }
}
