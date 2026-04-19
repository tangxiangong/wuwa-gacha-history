use dioxus::prelude::*;
use wuwa_gacha_history::{segments_by_five, CardPool, EnrichedPull, FiveStarSegment};

use crate::assets_wiki::{character_asset, weapon_asset};

#[derive(Props, Clone, PartialEq)]
pub struct CardsViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn CardsView(props: CardsViewProps) -> Element {
    let segs: Vec<FiveStarSegment> = segments_by_five(&props.pulls)
        .into_iter()
        .filter(|s| !s.pad)
        .collect();
    if segs.is_empty() {
        return rsx! { div { class: "text-fg-dim", "暂无 5★ 记录" } };
    }
    let mut display = segs;
    display.reverse();

    rsx! {
        div { class: "flex flex-wrap gap-4",
            for seg in display.into_iter() {
                Card { seg }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CardProps {
    seg: FiveStarSegment,
}

#[component]
fn Card(props: CardProps) -> Element {
    let Some(end) = props.seg.end.as_ref() else {
        return rsx! {};
    };
    let is_weapon = matches!(
        end.record.card_pool,
        CardPool::FeaturedWeaponConvene | CardPool::StandardWeaponConvene
    );
    let img = if is_weapon {
        weapon_asset(&end.record.name)
    } else {
        character_asset(&end.record.name)
    };
    let border = if props.seg.is_up { "border-star-5" } else { "border-star-4" };
    let name = end.record.name.clone();
    let pity_text = format!("{} 抽", props.seg.pity);
    let up_label = if props.seg.is_up { "UP" } else { "歪" };
    let up_class = if props.seg.is_up { "text-xs text-star-5" } else { "text-xs text-star-4" };
    let first_char_fallback: String = name.chars().next().map(|c| c.to_string()).unwrap_or_default();

    rsx! {
        div { class: "w-36 bg-bg-panel border-2 {border} rounded p-2 flex flex-col items-center gap-1",
            if let Some(a) = img {
                img { src: a, alt: "{name}", class: "w-32 h-32 object-cover rounded" }
            } else {
                div { class: "w-32 h-32 rounded bg-border-gold/20 flex items-center justify-center text-fg-dim",
                    "{first_char_fallback}"
                }
            }
            div { class: "text-sm text-star-5 truncate w-full text-center", "{name}" }
            div { class: "text-xs text-fg-dim", "{pity_text}" }
            div { class: "{up_class}", "{up_label}" }
        }
    }
}
