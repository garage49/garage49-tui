use std::sync::Arc;

use garage49_tui_iocraft::{Binding, ConfirmDialog, List, ListItem, MainFocus, MessageDialog, OverlayHandle, OverlayVariant, Palette, Selection, UseKeys, UseStatus};
use iocraft::prelude::*;

fn items() -> Vec<ListItem> {
    vec![
        ListItem::new("confirm", "Confirm dialog").section("Dialogs"),
        ListItem::new("danger", "Confirm dialog (destructive)").section("Dialogs"),
        ListItem::new("error", "Error dialog").section("Dialogs"),
        ListItem::new("message", "Message dialog").section("Dialogs"),
        ListItem::new("palette", "Command palette").shortcut("ctrl+p").section("Pop-ups"),
    ]
}

fn sample_commands() -> Vec<ListItem> {
    vec![
        ListItem::new("new", "New project").shortcut("ctrl+n").section("File"),
        ListItem::new("open", "Open…").shortcut("ctrl+o").section("File"),
        ListItem::new("rename", "Rename").section("Edit"),
        ListItem::new("delete", "Delete").section("Edit"),
        ListItem::new("shortcuts", "Keyboard shortcuts").shortcut("?").section("Help"),
    ]
}

#[component]
pub fn DialogsPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let overlay = *hooks.use_context::<OverlayHandle>();
    let selected = hooks.use_state(|| Some("confirm".to_string()));
    let all = items();
    let ids: Vec<String> = all.iter().map(|i| i.id.clone()).collect();
    let current = selected.read().clone();
    let done = move |text: &str| { overlay.hide(); status.report(text); };
    let open = move |item: ListItem| match item.id.as_str() {
        "confirm" => overlay.show(Arc::new(move || element! { ConfirmDialog(title: "Save changes", message: "Save the current project before closing it?", confirm_label: "Save".to_string(), on_confirm: move |_| done("saved"), on_cancel: move |_| done("cancelled")) }.into_any())),
        "danger" => overlay.show(Arc::new(move || element! { ConfirmDialog(danger: true, title: "Delete project", message: "Delete 한글 프로젝트 aterm? This cannot be undone.", confirm_label: "Delete".to_string(), on_confirm: move |_| done("deleted"), on_cancel: move |_| done("cancelled")) }.into_any())),
        "error" => overlay.show(Arc::new(move || element! { MessageDialog(variant: OverlayVariant::Error, title: "Connection failed", message: "Could not reach upstream (timeout after 30s). Check the network and try again.", on_close: move |_| done("error dismissed")) }.into_any())),
        "message" => overlay.show(Arc::new(move || element! { MessageDialog(title: "About garage49", message: "TUI design system sample · iocraft 0.9", on_close: move |_| done("closed")) }.into_any())),
        _ => overlay.show(Arc::new(move || element! { Palette(title: "Commands", items: sample_commands(), on_close: move |_| overlay.hide(), on_pick: move |picked: ListItem| done(&format!("palette: {}", picked.label))) }.into_any())),
    };
    let mover = |delta: i32| { let ids = ids.clone(); let current = current.clone(); move || { let mut selected = selected; selected.set(Selection::move_by(&ids, current.as_deref(), delta)); } };
    let enter = { let all = all.clone(); let current = current.clone(); move || { if let Some(item) = all.iter().find(|i| Some(&i.id) == current.as_ref()) { open(item.clone()) } } };
    hooks.use_keys(focused, vec![Binding::new(&["up", "k"], mover(-1)), Binding::new(&["down", "j"], mover(1)), Binding::new(&["enter"], enter)], None);
    let click = move |item: ListItem| { let mut selected = selected; selected.set(Some(item.id.clone())); open(item); };
    let wheel = move |item: ListItem| { let mut selected = selected; selected.set(Some(item.id)); };
    element! {
        View(flex_direction: FlexDirection::Column, width: 50) {
            List(items: all, selected_id: current, focused: focused, on_click: click, on_select: wheel)
        }
    }
}
