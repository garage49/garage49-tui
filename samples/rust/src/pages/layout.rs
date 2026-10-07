use garage49_tui_iocraft::{Binding, Form, Label, LabelVariant, List, ListItem, MainFocus, MasterDetail, Section, Select, Split, TextField, Toggle, UseKeys, UseStatus};
use iocraft::prelude::*;

const FIELDS: [&str; 5] = ["name", "url", "branch", "auto", "projects"];
const PROJECTS: usize = 4;

fn recent() -> Vec<ListItem> {
    vec![
        ListItem::new("r1", "garage49-tui").value("2 min ago"),
        ListItem::new("r2", "herdr-ranch").value("1 h ago"),
        ListItem::new("r3", "adoc").value("yesterday"),
    ]
}

fn shortcuts() -> Vec<ListItem> {
    vec![
        ListItem::new("s1", "Command palette").shortcut("ctrl+p"),
        ListItem::new("s2", "Help").shortcut("?"),
        ListItem::new("s3", "Quit").shortcut("q"),
    ]
}

fn details(project: &str) -> Vec<ListItem> {
    let (path, branch, sync) = match project {
        "r2" => ("~/work/herdr-ranch", "tui", "1 h ago"),
        "r3" => ("~/work/adoc", "main", "yesterday"),
        _ => ("~/work/garage49-tui", "main", "2 min ago"),
    };
    vec![ListItem::new("path", "Path").value(path), ListItem::new("branch", "Branch").value(branch), ListItem::new("sync", "Last sync").value(sync)]
}

/// Section, Form, Split and MasterDetail together: the layout every page is composed of.
#[component]
pub fn LayoutPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let field = hooks.use_state(|| 0usize);
    let project = hooks.use_state(|| 0usize);
    let name = hooks.use_state(|| "garage49-tui".to_string());
    let url = hooks.use_state(|| "https://github.com/garage49/garage49-tui".to_string());
    let branch = hooks.use_state(|| "main".to_string());
    let auto = hooks.use_state(|| true);
    let current = field.get();
    let row = project.get();
    let step = move |delta: i32| {
        let (mut field, mut project) = (field, project);
        if current == PROJECTS {
            let next = row as i32 + delta;
            if next < 0 { field.set(PROJECTS - 1) } else { project.set((next as usize).min(2)) }
        } else {
            field.set((current as i32 + delta).clamp(0, FIELDS.len() as i32 - 1) as usize);
        }
    };
    hooks.use_keys(focused, vec![Binding::new(&["up"], move || step(-1)), Binding::new(&["down"], move || step(1))], None);
    let is = |id: &str| focused && FIELDS[current] == id;
    let focus_on = |id: &'static str| move |_: ()| { let mut field = field; field.set(FIELDS.iter().position(|f| *f == id).unwrap_or(0)); };
    let pick = move |item: ListItem| {
        let (mut field, mut project) = (field, project);
        project.set(recent().iter().position(|p| p.id == item.id).unwrap_or(0));
        field.set(PROJECTS);
        status.report(&format!("project: {}", item.label));
    };
    let branches = vec![ListItem::new("main", "main"), ListItem::new("release", "release/0.2")];
    let selected = recent()[row].id.clone();
    element! {
        View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
            Section(title: "A form".to_string()) {
                Label(content: "A page is sections. One label column (as wide as the longest label), one value column: the URL field is as wide as the name field.", variant: LabelVariant::Muted)
                Form {
                    TextField(label: "Name", value: name.read().clone(), on_change: move |v: String| { let mut name = name; name.set(v) }, focused: is("name"), on_focus: focus_on("name"))
                    TextField(label: "Repository URL", value: url.read().clone(), on_change: move |v: String| { let mut url = url; url.set(v) }, focused: is("url"), on_focus: focus_on("url"))
                    Select(label: "Branch", options: branches, value: branch.read().clone(), on_change: move |o: ListItem| { let mut branch = branch; status.report(&format!("branch: {}", o.label)); branch.set(o.id) }, focused: is("branch"), on_focus: focus_on("branch"))
                    Toggle(label: "Auto-sync", value: auto.get(), on_change: move |v: bool| { let mut auto = auto; auto.set(v) }, focused: is("auto"), on_focus: focus_on("auto"))
                }
            }
            Split {
                Section(title: "Recent".to_string()) { List(items: recent(), focused: false) }
                Section(title: "Shortcuts".to_string()) { List(items: shortcuts(), focused: false) }
            }
            Section(title: "Projects".to_string()) {
                MasterDetail(items: recent(), selected_id: selected.clone(), focused: is("projects"), on_select: pick, on_click: pick) {
                    List(items: details(&selected), focused: false)
                }
            }
        }
    }
}
