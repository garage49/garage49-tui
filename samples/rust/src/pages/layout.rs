use garage49_tui_iocraft::{Binding, Form, Label, LabelVariant, List, ListItem, MainFocus, Section, Select, Split, TextField, Toggle, UseKeys, UseStatus};
use iocraft::prelude::*;

const FIELDS: [&str; 4] = ["name", "url", "branch", "auto"];

/// Section, Split and Form together: the layout every page is composed of.
#[component]
pub fn LayoutPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let field = hooks.use_state(|| 0usize);
    let name = hooks.use_state(|| "garage49-tui".to_string());
    let url = hooks.use_state(|| "https://github.com/garage49/garage49-tui".to_string());
    let branch = hooks.use_state(|| "main".to_string());
    let auto = hooks.use_state(|| true);
    let current = field.get();
    let step = move |delta: i32| { let mut field = field; field.set((current as i32 + delta).clamp(0, FIELDS.len() as i32 - 1) as usize); };
    hooks.use_keys(focused, vec![Binding::new(&["up"], move || step(-1)), Binding::new(&["down"], move || step(1))], None);
    let is = |id: &str| focused && FIELDS[current] == id;
    let focus_on = |id: &'static str| move |_: ()| { let mut field = field; field.set(FIELDS.iter().position(|f| *f == id).unwrap_or(0)); };
    let branches = vec![ListItem::new("main", "main"), ListItem::new("release", "release/0.2")];
    let recent = vec![
        ListItem::new("r1", "garage49-tui").value("2 min ago").section("Recent"),
        ListItem::new("r2", "herdr-ranch").value("1 h ago").section("Recent"),
        ListItem::new("r3", "adoc").value("yesterday").section("Recent"),
    ];
    element! {
        View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
            Section(title: "A section".to_string()) {
                Label(content: "A page is sections; a section is a heading and its body; one blank row between sections.", variant: LabelVariant::Muted)
            }
            Section(title: "A form".to_string()) {
                Label(content: "One label column (as wide as the longest label), one value column: the URL field is as wide as the name field.", variant: LabelVariant::Muted)
                Form {
                    TextField(label: "Name", value: name.read().clone(), on_change: move |v: String| { let mut name = name; name.set(v) }, focused: is("name"), on_focus: focus_on("name"))
                    TextField(label: "Repository URL", value: url.read().clone(), on_change: move |v: String| { let mut url = url; url.set(v) }, focused: is("url"), on_focus: focus_on("url"))
                    Select(label: "Branch", options: branches, value: branch.read().clone(), on_change: move |o: ListItem| { let mut branch = branch; status.report(&format!("branch: {}", o.label)); branch.set(o.id) }, focused: is("branch"), on_focus: focus_on("branch"))
                    Toggle(label: "Auto-sync", value: auto.get(), on_change: move |v: bool| { let mut auto = auto; auto.set(v) }, focused: is("auto"), on_focus: focus_on("auto"))
                }
            }
            Section(title: "A split".to_string()) {
                Label(content: "Sub-panes on alternating surfaces, one cell apart, padded 2×1.", variant: LabelVariant::Muted)
            }
            Split {
                View(flex_direction: FlexDirection::Column) {
                    Label(content: "Left pane", variant: LabelVariant::Heading)
                    List(items: recent, selected_id: "r1".to_string(), focused: false)
                }
                View(flex_direction: FlexDirection::Column) {
                    Label(content: "Right pane", variant: LabelVariant::Heading)
                    Label(content: "The right pane sits on the panel surface, so the edge shows without a line.")
                }
            }
        }
    }
}
