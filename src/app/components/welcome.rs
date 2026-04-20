use dioxus::prelude::*;

use crate::app::components::fetch_form::FetchForm;

#[derive(Props, Clone, PartialEq)]
pub struct WelcomePageProps {
    pub on_user_added: EventHandler<String>,
}

#[component]
pub fn WelcomePage(props: WelcomePageProps) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center p-6 bg-bg-base",
            div { class: "w-full max-w-xl bg-bg-panel border border-border-gold/50 rounded-lg p-8 flex flex-col gap-5",
                h2 { class: "text-xl font-semibold text-star-5", "欢迎使用鸣潮抽卡记录" }
                p { class: "text-fg-dim text-sm leading-relaxed",
                    "从「自动填充参数」三种方式任选其一获取 JSON 参数（包含 playerId / serverId / languageCode / recordId），或手动粘贴；系统会自动拉取所有卡池的记录。"
                }
                FetchForm {
                    on_success: move |pid: String| props.on_user_added.call(pid),
                }
            }
        }
    }
}
