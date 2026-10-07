use garage49_tui_iocraft::{Binding, Label, LabelVariant, MainFocus, Tab, Tabs, TabsSize, UseKeys, UseStatus};
use iocraft::prelude::*;

fn tabs() -> Vec<Tab> {
    vec![Tab::new("summary", "Summary"), Tab::new("files", "Files"), Tab::new("history", "기록")]
}

#[component]
pub fn TabsPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let active = hooks.use_state(|| "summary".to_string());
    let active_id = active.read().clone();
    let all = tabs();
    let index = all.iter().position(|t| t.id == active_id).unwrap_or(0);
    let len = all.len();
    let select = move |tab: Tab| { let mut active = active; status.report(&format!("tab: {}", tab.label)); active.set(tab.id); };
    let prev = { let all = all.clone(); move || select(all[(index + len - 1) % len].clone()) };
    let next = { let all = all.clone(); move || select(all[(index + 1) % len].clone()) };
    hooks.use_keys(focused, vec![Binding::new(&["left", "h"], prev), Binding::new(&["right", "l"], next)], None);
    let body = match active_id.as_str() {
        "files" => "Tabs switch with ←→ or hl; a click on a tab selects it.",
        "history" => "한글 탭 제목도 폭이 맞게 그려집니다.",
        _ => "Small tabs fill the active label; large tabs mark it with a thin line. Both: accent while focused.",
    };
    let small_select = select;
    let large_select = select;
    element! {
        View(flex_direction: FlexDirection::Column) {
            Label(content: "small · inside pages", variant: LabelVariant::Muted)
            Tabs(tabs: all.clone(), active_id: active_id.clone(), focused: focused, on_change: move |tab: Tab| small_select(tab))
            View(padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1) { Label(content: body) }
            View(height: 1)
            Label(content: "large · the top navigation", variant: LabelVariant::Muted)
            Tabs(tabs: all.clone(), active_id: active_id.clone(), focused: focused, size: TabsSize::Large, on_change: move |tab: Tab| large_select(tab))
        }
    }
}
