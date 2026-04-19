use dioxus::prelude::*;
use wuwa_gacha_history::{EnrichedPull, QualityLevel};

use crate::components::labels::{quality_label, quality_text_class};

#[derive(Props, Clone, PartialEq)]
pub struct RecordTableProps {
    pub pulls: Vec<EnrichedPull>,
    pub loading: bool,
}

#[component]
pub fn RecordTable(props: RecordTableProps) -> Element {
    if props.loading {
        return rsx! { div { class: "text-fg-dim py-4", "加载中…" } };
    }
    if props.pulls.is_empty() {
        return rsx! { div { class: "text-fg-dim py-4", "无数据" } };
    }
    rsx! {
        table { class: "w-full text-sm border-collapse",
            thead {
                tr { class: "text-fg-dim border-b border-border-gold/40 text-left",
                    th { class: "py-2 px-3", "名称" }
                    th { class: "py-2 px-3 w-16", "星级" }
                    th { class: "py-2 px-3 w-44", "时间" }
                    th { class: "py-2 px-3 w-16", "垫抽" }
                }
            }
            tbody {
                for p in props.pulls.iter() {
                    {
                        let time_str = p.record.time.format("%Y-%m-%d %H:%M:%S").to_string();
                        let name_cls = quality_text_class(p.record.quality_level);
                        let ql = quality_label(p.record.quality_level);
                        let is_five = p.record.quality_level == QualityLevel::FiveStar;
                        let pity = p.pity_at_pull.unwrap_or(0);
                        rsx! {
                            tr { class: "border-b border-border-gold/10 hover:bg-border-gold/5",
                                td { class: "py-1.5 px-3 {name_cls}", "{p.record.name}" }
                                td { class: "py-1.5 px-3", "{ql}" }
                                td { class: "py-1.5 px-3 text-fg-dim font-mono", "{time_str}" }
                                td { class: "py-1.5 px-3",
                                    if is_five { "{pity}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
