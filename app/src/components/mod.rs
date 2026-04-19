use dioxus::prelude::*;
use wuwa_gacha_history::CardPool;

pub mod add_user_dialog;
pub mod bars_view;
pub mod cards_view;
pub mod content_area;
pub mod fetch_form;
pub mod filter_panel;
pub mod labels;
pub mod main_layout;
pub mod pagination;
pub mod record_table;
pub mod sidebar;
pub mod summary_view;
pub mod welcome;

use add_user_dialog::AddUserDialog;
use main_layout::MainLayout;
use welcome::WelcomePage;

/// Reactive global state shared under `use_context`.
#[derive(Clone, Copy)]
pub struct GlobalState {
    pub users: Signal<Vec<String>>,
    pub player_id: Signal<Option<String>>,
    pub active_pool: Signal<Option<CardPool>>,
    pub add_user_open: Signal<bool>,
    pub export_open: Signal<bool>,
}

#[component]
pub fn Root() -> Element {
    let mut users = use_signal::<Vec<String>>(Vec::new);
    let mut player_id = use_signal::<Option<String>>(|| None);
    let active_pool = use_signal::<Option<CardPool>>(|| None);
    let mut add_user_open = use_signal(|| false);
    let export_open = use_signal(|| false);

    let state = GlobalState { users, player_id, active_pool, add_user_open, export_open };
    use_context_provider(|| state);

    // Initial load of users list.
    use_future(move || async move {
        match crate::api::list_users().await {
            Ok(list) => {
                if !list.is_empty() && player_id().is_none() {
                    player_id.set(Some(list[0].clone()));
                }
                users.set(list);
            }
            Err(e) => eprintln!("list_users failed: {e}"),
        }
    });

    let on_user_added = move |new_pid: String| {
        spawn(async move {
            if let Ok(list) = crate::api::list_users().await {
                users.set(list);
            }
            player_id.set(Some(new_pid));
        });
    };

    rsx! {
        if users().is_empty() {
            WelcomePage { on_user_added }
        } else {
            MainLayout {}
            AddUserDialog {
                open: add_user_open(),
                on_close: move |_| add_user_open.set(false),
                on_user_added,
            }
            // ExportDialog {} added in Task 24
        }
    }
}
