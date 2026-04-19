use dioxus::prelude::*;
use wuwa_gacha_history::CardPool;

use crate::components::{GlobalState, labels::card_pool_label};

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    pub on_add_user: EventHandler<()>,
    pub on_export: EventHandler<()>,
}

const GROUPS: &[(&str, &[CardPool])] = &[
    (
        "限定池",
        &[
            CardPool::FeaturedResonatorConvene,
            CardPool::FeaturedWeaponConvene,
        ],
    ),
    (
        "常驻池",
        &[
            CardPool::StandardResonatorConvene,
            CardPool::StandardWeaponConvene,
        ],
    ),
    (
        "其他",
        &[
            CardPool::NoviceConvene,
            CardPool::BeginnerChoiceConvene,
            CardPool::GivebackCustomConvene,
        ],
    ),
];

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let mut state = use_context::<GlobalState>();

    rsx! {
        aside { class: "w-40 min-w-40 h-screen bg-bg-panel border-r border-border-gold/40 flex flex-col",
            // User selector
            div { class: "p-3 border-b border-border-gold/30",
                select {
                    class: "w-full bg-bg-base border border-border-gold/60 rounded px-2 py-1 text-fg-base text-sm",
                    value: state.player_id.read().clone().unwrap_or_default(),
                    onchange: move |e| state.player_id.set(Some(e.value())),
                    for u in state.users.read().iter() {
                        option { value: "{u}", "{u}" }
                    }
                }
            }

            // Pool groups
            nav { class: "flex-1 overflow-y-auto p-2 flex flex-col gap-3",
                for (title, pools) in GROUPS.iter() {
                    div { class: "flex flex-col gap-1",
                        div { class: "text-xs text-fg-dim px-2", "{title}" }
                        for p in pools.iter().copied() {
                            PoolItem { pool: p }
                        }
                    }
                }
            }

            // Footer actions
            div { class: "p-2 flex flex-col gap-2 border-t border-border-gold/30",
                button {
                    class: "w-full bg-border-gold/80 text-bg-base text-sm py-1.5 rounded hover:bg-border-gold transition-colors",
                    onclick: move |_| props.on_add_user.call(()),
                    "添加用户"
                }
                button {
                    class: "w-full border border-border-gold text-star-5 text-sm py-1.5 rounded hover:bg-border-gold/20 transition-colors",
                    onclick: move |_| props.on_export.call(()),
                    "导出"
                }
            }
        }
    }
}

#[component]
fn PoolItem(pool: CardPool) -> Element {
    let mut state = use_context::<GlobalState>();
    let active = *state.active_pool.read() == Some(pool);
    let cls = if active {
        "px-2 py-1 rounded text-sm bg-border-gold/30 text-star-5 text-left"
    } else {
        "px-2 py-1 rounded text-sm text-fg-base hover:bg-border-gold/10 text-left"
    };
    rsx! {
        button { class: "{cls}",
            onclick: move |_| state.active_pool.set(Some(pool)),
            "{card_pool_label(pool)}"
        }
    }
}
