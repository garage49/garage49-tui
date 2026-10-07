use garage49_tui_iocraft::{Binding, Label, LabelVariant, ListItem, MainFocus, MouseState, Select, TextField, Theme, ThemeSwitch, Toggle, UseKeys, UseStatus};
use iocraft::prelude::*;

const FIELDS: [&str; 4] = ["theme", "mouse", "vim", "minsize"];

#[component]
pub fn SettingsPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let theme = hooks.use_context::<Theme>().clone();
    let switch = *hooks.use_context::<ThemeSwitch>();
    let mouse = *hooks.use_context::<MouseState>();
    let field = hooks.use_state(|| 0usize);
    let vim = hooks.use_state(|| true);
    let min_size = hooks.use_state(|| "80×24".to_string());
    let current = field.get();
    let step = move |delta: i32| { let mut field = field; field.set((current as i32 + delta).clamp(0, FIELDS.len() as i32 - 1) as usize); };
    hooks.use_keys(focused, vec![Binding::new(&["up"], move || step(-1)), Binding::new(&["down"], move || step(1))], None);
    let is = |id: &str| focused && FIELDS[current] == id;
    let focus_on = |id: &'static str| move |_: ()| { let mut field = field; field.set(FIELDS.iter().position(|f| *f == id).unwrap_or(0)); };
    let themes = vec![ListItem::new("opencode", "opencode (truecolor)"), ListItem::new("system", "system (ANSI 16)")];
    let theme_id = theme.name.trim_end_matches("-dimmed").to_string();
    element! {
        View(flex_direction: FlexDirection::Column, width: 60) {
            Label(content: "Appearance", variant: LabelVariant::Heading)
            Select(label: "Theme", options: themes, value: theme_id, focused: is("theme"), on_focus: focus_on("theme"),
                on_change: move |o: ListItem| { let next = if o.id == "system" { Theme::system() } else { Theme::opencode() }; status.report(&format!("theme: {}", o.label)); let mut t = switch.theme; t.set(next); })
            View(height: 1)
            Label(content: "Input", variant: LabelVariant::Heading)
            Toggle(label: "Mouse", value: mouse.enabled.get(), focused: is("mouse"), on_focus: focus_on("mouse"),
                on_change: move |v: bool| { let mut enabled = mouse.enabled; enabled.set(v); status.report(if v { "mouse: on" } else { "mouse: off" }); })
            Toggle(label: "Vim keys", value: vim.get(), focused: is("vim"), on_focus: focus_on("vim"),
                on_change: move |v: bool| { let mut vim = vim; vim.set(v); status.report(if v { "vim keys: on" } else { "vim keys: off" }); })
            View(height: 1)
            Label(content: "Window", variant: LabelVariant::Heading)
            TextField(label: "Minimum size", value: min_size.read().clone(), focused: is("minsize"), on_focus: focus_on("minsize"), on_change: move |v: String| { let mut min_size = min_size; min_size.set(v) })
            View(height: 1)
            Label(content: "Theme and mouse are live; the other values are only reported. With the mouse on, shift+drag selects text in most terminals; `m` toggles it anywhere.", variant: LabelVariant::Muted)
        }
    }
}
