use iocraft::prelude::*;

use super::focus::{FocusState, UseFocusRegion};
use crate::components::{FocusRegion, List, ListItem, OverlayHandle, Panel, RegionSurface, Selection};
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};

#[derive(Default, Props)]
pub struct SidebarProps {
    pub items: Vec<ListItem>,
    pub selected_id: Option<String>,
    pub on_select: HandlerMut<'static, ListItem>,
    pub width: Option<u16>,
}

/// The App's left panel: a sectioned list that is a focus region. ↑↓ (jk) move the selection; enter goes down to the next region.
#[component]
pub fn Sidebar(props: &mut SidebarProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (id, focused) = hooks.use_focus_region(true);
    let overlay_open = hooks.use_context::<OverlayHandle>().is_open();
    let active = focused && !overlay_open;
    let focus = hooks.use_context::<FocusState>().clone();
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let focus = focus.clone();
        hooks.use_mouse(allowed, move |event| {
            if matches!(event.kind, MouseEventKind::Down(_)) {
                focus.focus(id);
            }
        });
    }
    let items = props.items.clone();
    let ids: Vec<String> = items.iter().map(|item| item.id.clone()).collect();
    let selected = props.selected_id.clone();
    let on_select = std::sync::Arc::new(std::sync::Mutex::new(props.on_select.take()));
    let mover = |delta: i32| {
        let on_select = on_select.clone();
        let items = items.clone();
        let ids = ids.clone();
        let selected = selected.clone();
        move || {
            if let Some(next) = Selection::move_by(&ids, selected.as_deref(), delta) {
                if let Some(item) = items.iter().find(|item| item.id == next) {
                    (on_select.lock().expect("handler"))(item.clone());
                }
            }
        }
    };
    hooks.use_keys(active, vec![Binding::new(&["up", "k"], mover(-1)), Binding::new(&["down", "j"], mover(1))], None);
    let click = {
        let on_select = on_select.clone();
        move |item: ListItem| (on_select.lock().expect("handler"))(item)
    };
    let wheel = {
        let on_select = on_select.clone();
        move |item: ListItem| (on_select.lock().expect("handler"))(item)
    };
    let width = props.width.unwrap_or(26);
    // The root is a View: iocraft delivers local mouse events only to components whose root node is a View.
    element! {
        View(flex_direction: FlexDirection::Row) {
            FocusRegion(focused: active, surface: RegionSurface::Panel) {
                Panel(width: width, grow: true) {
                    List(items: props.items.clone(), selected_id: props.selected_id.clone(), focused: focused, on_click: click, on_select: wheel)
                }
            }
        }
    }
}
