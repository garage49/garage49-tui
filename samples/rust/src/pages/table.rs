use garage49_tui_iocraft::{Binding, Column, MainFocus, Row, Table, UseKeys, UseStatus};
use iocraft::prelude::*;

fn rows() -> Vec<Row> {
    vec![
        Row::new("garage49-tui", &["garage49-tui", "TypeScript · Rust", "24", "active"]),
        Row::new("한글 프로젝트 aterm", &["한글 프로젝트 aterm", "TypeScript", "1210", "active"]),
        Row::new("skills", &["skills", "Markdown", "88", "idle"]),
        Row::new("herdr-connect", &["herdr-connect", "Go", "310", "archived"]),
        Row::new("voice-bridge 🎙", &["voice-bridge 🎙", "Python", "57", "idle"]),
    ]
}

#[component]
pub fn TablePage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let selected = hooks.use_state(|| Some("garage49-tui".to_string()));
    let all = rows();
    let current = selected.read().clone();
    let index = all.iter().position(|r| Some(&r.id) == current.as_ref()).unwrap_or(0);
    let mover = |delta: i32| { let all = all.clone(); move || { let i = (index as i32 + delta).clamp(0, all.len() as i32 - 1) as usize; let mut selected = selected; selected.set(Some(all[i].id.clone())); } };
    let open = { let current = current.clone(); move || { if let Some(id) = &current { status.report(&format!("open: {id}")); } } };
    hooks.use_keys(focused, vec![Binding::new(&["up", "k"], mover(-1)), Binding::new(&["down", "j"], mover(1)), Binding::new(&["enter"], open)], None);
    let columns = vec![Column::new("Name", 24), Column::new("Language", 18), Column::new("Files", 6).right(), Column::new("Status", 10)];
    let click = move |row: Row| { let mut selected = selected; selected.set(Some(row.id.clone())); status.report(&format!("open: {}", row.id)); };
    let wheel = move |row: Row| { let mut selected = selected; selected.set(Some(row.id)); };
    element! {
        View(flex_direction: FlexDirection::Column) {
            Table(columns: columns, rows: all, selected_id: current, focused: focused, on_click: click, on_select: wheel)
        }
    }
}
