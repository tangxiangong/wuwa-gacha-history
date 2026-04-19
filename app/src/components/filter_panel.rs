use dioxus::prelude::*;
use wuwa_gacha_history::QualityLevel;

#[derive(Props, Clone, PartialEq)]
pub struct FilterPanelProps {
    pub quality: Signal<Option<QualityLevel>>,
    pub name_query: Signal<String>,
    pub time_from: Signal<String>,
    pub time_to: Signal<String>,
    pub open: bool,
}

#[component]
pub fn FilterPanel(mut props: FilterPanelProps) -> Element {
    if !props.open {
        return rsx! {};
    }
    rsx! {
        div { class: "flex flex-wrap gap-3 items-end p-3 bg-bg-panel/60 border border-border-gold/30 rounded",
            // Quality chips
            div { class: "flex gap-2 items-center",
                span { class: "text-fg-dim text-sm", "星级：" }
                QualityChip { level: None,                           quality: props.quality }
                QualityChip { level: Some(QualityLevel::FiveStar),  quality: props.quality }
                QualityChip { level: Some(QualityLevel::FourStar),  quality: props.quality }
                QualityChip { level: Some(QualityLevel::ThreeStar), quality: props.quality }
            }

            // Name
            label { class: "text-sm flex items-center gap-2",
                span { class: "text-fg-dim", "名称：" }
                input {
                    class: "bg-bg-base border border-border-gold/40 rounded px-2 py-1 text-sm",
                    value: "{props.name_query}",
                    oninput: move |e| props.name_query.set(e.value()),
                }
            }

            // Time range (YYYY-MM-DD)
            label { class: "text-sm flex items-center gap-2",
                span { class: "text-fg-dim", "起：" }
                input {
                    r#type: "text",
                    placeholder: "2025-01-01",
                    class: "bg-bg-base border border-border-gold/40 rounded px-2 py-1 text-sm w-28",
                    value: "{props.time_from}",
                    oninput: move |e| props.time_from.set(e.value()),
                }
            }
            label { class: "text-sm flex items-center gap-2",
                span { class: "text-fg-dim", "止：" }
                input {
                    r#type: "text",
                    placeholder: "2026-04-19",
                    class: "bg-bg-base border border-border-gold/40 rounded px-2 py-1 text-sm w-28",
                    value: "{props.time_to}",
                    oninput: move |e| props.time_to.set(e.value()),
                }
            }
        }
    }
}

#[component]
fn QualityChip(level: Option<QualityLevel>, mut quality: Signal<Option<QualityLevel>>) -> Element {
    let is_active = *quality.read() == level;
    let label = match level {
        None => "全部",
        Some(QualityLevel::FiveStar) => "5★",
        Some(QualityLevel::FourStar) => "4★",
        Some(QualityLevel::ThreeStar) => "3★",
    };
    let cls = if is_active {
        "px-2 py-1 rounded bg-border-gold text-bg-base text-sm"
    } else {
        "px-2 py-1 rounded border border-border-gold/40 text-fg-base hover:bg-border-gold/10 text-sm"
    };
    rsx! {
        button { class: "{cls}",
            onclick: move |_| quality.set(level),
            "{label}"
        }
    }
}
