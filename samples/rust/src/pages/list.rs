use garage49_tui_iocraft::{Binding, List, ListItem, MainFocus, Selection, UseKeys, UseStatus, Section};
use iocraft::prelude::*;

fn items() -> Vec<ListItem> {
    vec![
        ListItem::new("r1", "garage49-tui").value("2 min ago").section("Recent projects"),
        ListItem::new("r2", "한글 프로젝트 aterm").value("yesterday").section("Recent projects"),
        ListItem::new("r3", "skills").value("3 days ago").section("Recent projects"),
        ListItem::new("a1", "Open a project").shortcut("ctrl+o").section("Actions"),
        ListItem::new("a2", "Command palette").shortcut("ctrl+p").section("Actions"),
        ListItem::new("a3", "A very long label that does not fit in the list width and gets truncated").section("Actions"),
        ListItem::new("s1", "Theme").value("opencode").section("Settings"),
        ListItem::new("s2", "Mouse").value("on").section("Settings"),
    ]
}

#[component]
pub fn ListPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let items = items();
    let ids: Vec<String> = items.iter().map(|i| i.id.clone()).collect();
    let selected = hooks.use_state(|| Some("r1".to_string()));
    let current = selected.read().clone();
    let mover = |delta: i32| {
        let ids = ids.clone();
        let current = current.clone();
        let mut selected = selected;
        move || selected.set(Selection::move_by(&ids, current.as_deref(), delta))
    };
    let activate = {
        let items = items.clone();
        let current = current.clone();
        move || {
            if let Some(item) = items.iter().find(|i| Some(&i.id) == current.as_ref()) {
                status.report(&format!("activated: {}", item.label));
            }
        }
    };
    hooks.use_keys(focused, vec![Binding::new(&["up", "k"], mover(-1)), Binding::new(&["down", "j"], mover(1)), Binding::new(&["enter"], activate)], None);
    let mut select_state = selected;
    let mut click_state = selected;
    element! {
        Section(title: "Recent projects and actions".to_string()) {
        View(flex_direction: FlexDirection::Column, width: 56) {
            List(items: items, selected_id: current, focused: focused,
                on_click: move |item: ListItem| { click_state.set(Some(item.id.clone())); status.report(&format!("activated: {}", item.label)); },
                on_select: move |item: ListItem| select_state.set(Some(item.id)))
        }
        }
    }
}
