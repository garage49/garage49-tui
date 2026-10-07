use garage49_tui_iocraft::{App, Command, Content, HelpEntry, KeyHint, ListItem, Main, Nav, Sidebar, StatusSegment, StatusTone, Tab};
use iocraft::prelude::*;

use crate::pages::{DialogsPage, EditingPage, FormPage, HelpPage, HomePage, IndicatorsPage, LabelsPage, LayoutPage, ListPage, LogPage, SettingsPage, TablePage, TabsPage, TreePage};

struct Page {
    item: ListItem,
    title: &'static str,
}

fn pages() -> Vec<Page> {
    vec![
        Page { item: ListItem::new("home", "Home").section("Overview"), title: "Home" },
        Page { item: ListItem::new("labels", "Labels & tokens").section("Overview"), title: "Labels and color tokens" },
        Page { item: ListItem::new("layout", "Layout").section("Overview"), title: "Layout · sections, forms and splits" },
        Page { item: ListItem::new("tabs", "Tabs").section("Navigation"), title: "Tabs" },
        Page { item: ListItem::new("tree", "Tree view").section("Navigation"), title: "Tree view" },
        Page { item: ListItem::new("list", "List").section("Data"), title: "List" },
        Page { item: ListItem::new("table", "Table").section("Data"), title: "Table" },
        Page { item: ListItem::new("log", "Log view").section("Data"), title: "Log view" },
        Page { item: ListItem::new("form", "Form fields").section("Input"), title: "Form fields" },
        Page { item: ListItem::new("editing", "Text editing").section("Input"), title: "Text editing · text area and input box" },
        Page { item: ListItem::new("dialogs", "Dialogs & pop-ups").section("Feedback"), title: "Dialogs and pop-ups" },
        Page { item: ListItem::new("indicators", "Chips & spinner").section("Feedback"), title: "Chips and spinners" },
    ]
}

fn gallery_help() -> Vec<HelpEntry> {
    vec![HelpEntry::new("←→ / hl", "switch the view while the navigation is focused")]
}

fn views() -> Vec<Tab> {
    vec![Tab::new("gallery", "Gallery"), Tab::new("dashboard", "Dashboard"), Tab::new("settings", "Settings"), Tab::new("help", "Help")]
}

#[component]
pub fn Gallery(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let view = hooks.use_state(|| "gallery".to_string());
    let page = hooks.use_state(|| "home".to_string());
    let view_id = view.read().clone();
    let page_id = page.read().clone();
    let pages = pages();
    let has_sidebar = view_id == "gallery";
    let (title, body): (String, AnyElement<'static>) = if has_sidebar {
        let current = pages.iter().find(|p| p.item.id == page_id).unwrap_or(&pages[0]);
        let body = match current.item.id.as_str() {
            "labels" => element!(LabelsPage).into_any(),
            "layout" => element!(LayoutPage).into_any(),
            "tabs" => element!(TabsPage).into_any(),
            "tree" => element!(TreePage).into_any(),
            "list" => element!(ListPage).into_any(),
            "table" => element!(TablePage).into_any(),
            "log" => element!(LogPage).into_any(),
            "form" => element!(FormPage).into_any(),
            "editing" => element!(EditingPage).into_any(),
            "dialogs" => element!(DialogsPage).into_any(),
            "indicators" => element!(IndicatorsPage).into_any(),
            _ => element!(HomePage).into_any(),
        };
        (current.title.to_string(), body)
    } else {
        match view_id.as_str() {
            "settings" => ("Settings".to_string(), element!(SettingsPage).into_any()),
            "help" => ("Help".to_string(), element!(HelpPage(entries: gallery_help())).into_any()),
            _ => ("Dashboard · a view without a sidebar".to_string(), element!(HomePage).into_any()),
        }
    };
    let items: Vec<ListItem> = pages.iter().map(|p| p.item.clone()).collect();
    let mut commands: Vec<Command> = views().into_iter().map(|v| {
        let id = v.id.clone();
        Command::new(ListItem::new(&format!("view:{}", v.id), &format!("Go to {}", v.label)).section("Views"), move || { let mut view = view; view.set(id.clone()) })
    }).collect();
    commands.extend(pages.iter().map(|p| {
        let id = p.item.id.clone();
        Command::new(ListItem::new(&format!("page:{}", p.item.id), &format!("Open {}", p.item.label)).section("Gallery"), move || { let mut view = view; let mut page = page; view.set("gallery".into()); page.set(id.clone()); })
    }));
    let page_label = pages.iter().find(|p| p.item.id == page_id).map(|p| p.item.label.clone()).unwrap_or_default();
    let view_label = views().iter().find(|v| v.id == view_id).map(|v| v.label.clone()).unwrap_or_default();
    let mut view_state = view;
    let mut page_state = page;
    element! {
        App(commands: commands, help: gallery_help(), hints: vec![KeyHint::new("g", "home").with_action(move || { let mut view = view; let mut page = page; view.set("gallery".into()); page.set("home".into()); })],
            status: vec![StatusSegment::new(&view_label, StatusTone::Muted), StatusSegment::new(&page_label, StatusTone::Muted)],
            context: "garage49 · ~/work/garage49-tui", quit_message: "Quit garage49?".to_string()) {
            Nav(brand: "garage49", items: views(), active_id: view_id.clone(), on_change: move |tab: Tab| view_state.set(tab.id), right: "v0.1.0".to_string())
            Content {
                #(if has_sidebar { Some(element! { Sidebar(items: items.clone(), selected_id: page_id.clone(), on_select: move |item: ListItem| page_state.set(item.id)) }) } else { None })
                Main(title: title) { #(std::iter::once(body)) }
            }
        }
    }
}
