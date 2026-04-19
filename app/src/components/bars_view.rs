use dioxus::prelude::*;
use wuwa_gacha_history::{segments_by_five, EnrichedPull, FiveStarSegment};

#[derive(Props, Clone, PartialEq)]
pub struct BarsViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn BarsView(props: BarsViewProps) -> Element {
    let segs = segments_by_five(&props.pulls);
    if segs.is_empty() {
        return rsx! { div { class: "text-fg-dim", "暂无 5★ 段" } };
    }
    // Most recent 5★ first
    let mut display: Vec<FiveStarSegment> = segs.clone();
    display.reverse();

    rsx! {
        div { class: "flex flex-col gap-2",
            for seg in display.into_iter() {
                SegmentBar { seg }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SegmentBarProps {
    seg: FiveStarSegment,
}

#[component]
fn SegmentBar(props: SegmentBarProps) -> Element {
    let seg = &props.seg;

    let (bar_class, label) = if seg.pad {
        ("bg-fg-dim/40", format!("当前累计 {} 抽未出 5★", seg.pity))
    } else if seg.is_up {
        (
            "bg-star-5/80",
            seg.end.as_ref().map(|e| format!("{} — {} 抽（UP）", e.record.name, seg.pity))
                .unwrap_or_default(),
        )
    } else {
        (
            "bg-star-4/80",
            seg.end.as_ref().map(|e| format!("{} — {} 抽（歪）", e.record.name, seg.pity))
                .unwrap_or_default(),
        )
    };

    let width_pct = (seg.pity as f64 / 80.0 * 100.0).min(100.0);
    let width_style = format!("width: {:.1}%;", width_pct);

    rsx! {
        div { class: "flex items-center gap-3",
            div { class: "flex-1 bg-bg-panel rounded h-6 relative overflow-hidden border border-border-gold/30",
                div {
                    class: "h-full {bar_class}",
                    style: "{width_style}",
                }
            }
            div { class: "w-80 text-xs text-fg-base", "{label}" }
        }
    }
}
