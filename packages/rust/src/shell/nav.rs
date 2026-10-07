use iocraft::prelude::*;

use super::focus::{FocusState, UseFocusRegion};
use crate::components::{FocusRegion, OverlayHandle, RegionSurface, Tab, TopNav};
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};

#[derive(Default, Props)]
pub struct NavProps {
    pub brand: String,
    pub items: Vec<Tab>,
    pub active_id: String,
    pub on_change: HandlerMut<'static, Tab>,
    pub right: Option<String>,
}

/// The App's top navigation: a focus region whose ←→ (hl) switch the active item. Enter goes down to the next region.
#[component]
pub fn Nav(props: &mut NavProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
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
    let index = items.iter().position(|item| item.id == props.active_id).unwrap_or(0);
    let on_change = std::sync::Arc::new(std::sync::Mutex::new(props.on_change.take()));
    let len = items.len().max(1);
    let left = {
        let on_change = on_change.clone();
        let items = items.clone();
        move || {
            if let Some(item) = items.get((index + len - 1) % len) {
                (on_change.lock().expect("handler"))(item.clone());
            }
        }
    };
    let right = {
        let on_change = on_change.clone();
        let items = items.clone();
        move || {
            if let Some(item) = items.get((index + 1) % len) {
                (on_change.lock().expect("handler"))(item.clone());
            }
        }
    };
    hooks.use_keys(active, vec![Binding::new(&["left", "h"], left), Binding::new(&["right", "l"], right)], None);
    let on_click = {
        let on_change = on_change.clone();
        let focus = focus.clone();
        move |item: Tab| {
            focus.focus(id);
            (on_change.lock().expect("handler"))(item);
        }
    };
    // The root is a View: iocraft delivers local mouse events only to components whose root node is a View.
    element! {
        View(flex_direction: FlexDirection::Column, width: 100pct) {
            FocusRegion(focused: active, surface: RegionSurface::Heading, rows: 1u16) {
                TopNav(brand: props.brand.clone(), items: items.clone(), active_id: props.active_id.clone(), focused: active, on_change: on_click, right: props.right.clone())
            }
        }
    }
}
