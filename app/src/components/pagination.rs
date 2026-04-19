use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    pub page: usize,        // 1-based current page
    pub total_pages: usize, // >= 1
    pub on_page: EventHandler<usize>,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    if props.total_pages <= 1 {
        return rsx! {};
    }
    let page = props.page.max(1).min(props.total_pages);
    let win = window(page, props.total_pages, 5);
    let prev_disabled = page <= 1;
    let next_disabled = page >= props.total_pages;
    let on_page = props.on_page.clone();

    rsx! {
        nav { class: "flex gap-1 items-center justify-center py-3 text-sm",
            PageBtn {
                label: "‹".to_string(),
                disabled: prev_disabled,
                on_click: move |_| on_page.call(page - 1),
            }
            for p in win.iter().copied() {
                PageBtn {
                    label: format!("{p}"),
                    active: p == page,
                    on_click: {
                        let on_page = props.on_page.clone();
                        move |_| on_page.call(p)
                    },
                }
            }
            PageBtn {
                label: "›".to_string(),
                disabled: next_disabled,
                on_click: {
                    let on_page = props.on_page.clone();
                    move |_| on_page.call(page + 1)
                },
            }
        }
    }
}

fn window(page: usize, total: usize, span: usize) -> Vec<usize> {
    let half = span / 2;
    let mut start = page.saturating_sub(half).max(1);
    let mut end = start + span - 1;
    if end > total {
        end = total;
        start = (end + 1).saturating_sub(span).max(1);
    }
    (start..=end).collect()
}

#[derive(Props, Clone, PartialEq)]
struct PageBtnProps {
    label: String,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    active: bool,
    on_click: EventHandler<()>,
}

#[component]
fn PageBtn(props: PageBtnProps) -> Element {
    let mut cls = String::from("px-2 py-1 rounded border border-border-gold/40 ");
    if props.active {
        cls.push_str("bg-border-gold text-bg-base");
    } else if props.disabled {
        cls.push_str("text-fg-dim opacity-40 cursor-not-allowed");
    } else {
        cls.push_str("text-fg-base hover:bg-border-gold/20");
    }
    rsx! {
        button { class: "{cls}", disabled: props.disabled,
            onclick: move |_| props.on_click.call(()),
            "{props.label}"
        }
    }
}
