use garage49_tui_iocraft::{Binding, Checkbox, Form, Label, LabelVariant, ListItem, MainFocus, RadioGroup, Section, Select, TextField, Toggle, UseKeys, UseStatus};
use iocraft::prelude::*;

const FIELDS: [&str; 10] = ["name", "path", "theme", "language", "layout", "mouse", "vim", "snapshot", "cjk", "restore"];

#[component]
pub fn FormPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let field = hooks.use_state(|| 0usize);
    let name = hooks.use_state(|| "garage49-tui".to_string());
    let path = hooks.use_state(String::new);
    let theme = hooks.use_state(|| "opencode".to_string());
    let language = hooks.use_state(|| "ts".to_string());
    let layout = hooks.use_state(|| "both".to_string());
    let mouse = hooks.use_state(|| true);
    let vim = hooks.use_state(|| true);
    let checks = hooks.use_state(|| [true, true, false]);
    let current = field.get();
    let step = move |delta: i32| { let mut field = field; field.set((current as i32 + delta).clamp(0, FIELDS.len() as i32 - 1) as usize); };
    hooks.use_keys(focused && FIELDS[current] != "layout", vec![Binding::new(&["up"], move || step(-1)), Binding::new(&["down"], move || step(1))], None);
    let is = |id: &str| focused && FIELDS[current] == id;
    let focus_on = |id: &'static str| move |_: ()| { let mut field = field; field.set(FIELDS.iter().position(|f| *f == id).unwrap_or(0)); };
    let themes = vec![ListItem::new("opencode", "opencode"), ListItem::new("system", "system (ANSI 16)"), ListItem::new("light", "light (coming later)")];
    let languages = vec![ListItem::new("ts", "TypeScript"), ListItem::new("rust", "Rust"), ListItem::new("go", "Go"), ListItem::new("ko", "한국어")];
    let layouts = vec![ListItem::new("top", "Top navigation only"), ListItem::new("sidebar", "Sidebar only"), ListItem::new("both", "Both · 상단 + 사이드바")];
    let check_labels = ["Snapshot tests", "CJK width test · 한글 폭", "Terminal restore test"];
    let check_ids = ["snapshot", "cjk", "restore"];
    let checks_now = checks.get();
    element! {
        View(flex_direction: FlexDirection::Column, flex_shrink: 0.0_f32) {
            Label(content: "↑↓ fields (and radio options) · space chooses/toggles/opens · ←→ cycles a select", variant: LabelVariant::Muted)
            View(height: 1)
            Section(title: "Project".to_string()) {
            Form {
            TextField(label: "Project name", value: name.read().clone(), on_change: move |v: String| { let mut name = name; name.set(v) }, focused: is("name"), on_focus: focus_on("name"))
            TextField(label: "Path", value: path.read().clone(), placeholder: "~/work/…".to_string(), on_change: move |v: String| { let mut path = path; path.set(v) }, focused: is("path"), on_focus: focus_on("path"))
            Select(label: "Theme", options: themes, value: theme.read().clone(), on_change: move |o: ListItem| { let mut theme = theme; status.report(&format!("theme: {}", o.label)); theme.set(o.id) }, focused: is("theme"), on_focus: focus_on("theme"))
            Select(label: "Language", options: languages, value: language.read().clone(), on_change: move |o: ListItem| { let mut language = language; status.report(&format!("language: {}", o.label)); language.set(o.id) }, focused: is("language"), on_focus: focus_on("language"))
            RadioGroup(label: "Layout", options: layouts, value: layout.read().clone(), on_change: move |o: ListItem| { let mut layout = layout; status.report(&format!("layout: {}", o.label)); layout.set(o.id) }, focused: is("layout"), on_focus: focus_on("layout"), on_leave: move |d: i32| step(d))
            Toggle(label: "Mouse", value: mouse.get(), on_change: move |v: bool| { let mut mouse = mouse; mouse.set(v) }, focused: is("mouse"), on_focus: focus_on("mouse"))
            Toggle(label: "Vim keys", value: vim.get(), on_change: move |v: bool| { let mut vim = vim; vim.set(v) }, focused: is("vim"), on_focus: focus_on("vim"))
            }
            }
            Section(title: "Tests to run".to_string()) {
            #((0..3).map(|i| {
                let id = check_ids[i];
                element! {
                    Checkbox(key: id, label: check_labels[i], checked: checks_now[i], focused: is(id), on_focus: focus_on(id),
                        on_change: move |v: bool| { let mut next = checks_now; next[i] = v; let mut checks = checks; checks.set(next); status.report(&format!("{id}: {}", if v { "checked" } else { "unchecked" })); })
                }
            }))
            }
        }
    }
}
