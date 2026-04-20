use crate::app::components::fetch_form::FetchForm;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WelcomePageProps {
    pub on_user_added: EventHandler<String>,
}

#[component]
pub fn WelcomePage(props: WelcomePageProps) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center justify-center p-8 gap-6 bg-bg-base",
            h1 { class: "text-2xl font-semibold text-star-5", "鸣潮抽卡记录" }
            p { class: "text-fg-dim max-w-md text-center",
                "首次使用：请在游戏内打开抽卡记录链接，从浏览器地址栏参数里复制 playerId / serverId / languageCode / recordId，拼成 JSON 粘贴下面后拉取。"
            }
            FetchForm {
                on_success: move |pid: String| props.on_user_added.call(pid),
            }
        }
    }
}
