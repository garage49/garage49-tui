use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::list::{List, ListItem};
use super::selection::Selection;
use crate::input::{Binding, UseKeys};
use crate::theme::{TextWidth, Theme};

#[derive(Default, Props)]
pub struct DropdownProps {
    pub items: Vec<ListItem>,
    /// The row the cursor starts on; the first row otherwise.
    pub initial_id: Option<String>,
    pub on_pick: HandlerMut<'static, ListItem>,
    pub on_close: HandlerMut<'static, ()>,
}

/// A list popover on a raised surface, under a select box. Owns its cursor and keys.
#[component]
pub fn Dropdown(props: &mut DropdownProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let initial = props.initial_id.clone();
    let cursor = hooks.use_state(move || initial);
    let ids: Vec<String> = props.items.iter().map(|i| i.id.clone()).collect();
    let current = Selection::ensure(&ids, cursor.read().as_deref());
    let on_pick = Arc::new(Mutex::new(props.on_pick.take()));
    let mut on_close = props.on_close.take();
    let mover = |delta: i32| { let ids = ids.clone(); let mut cursor = cursor; move || { let now = Selection::ensure(&ids, cursor.read().as_deref()); cursor.set(Selection::move_by(&ids, now.as_deref(), delta)) } };
    let pick = { let on_pick = on_pick.clone(); let items = props.items.clone(); let current = current.clone(); move || { if let Some(item) = items.iter().find(|i| Some(&i.id) == current.as_ref()) { (on_pick.lock().expect("handler"))(item.clone()); } } };
    hooks.use_keys(true, vec![Binding::new(&["esc"], move || on_close(())), Binding::new(&["up"], mover(-1)), Binding::new(&["down"], mover(1)), Binding::new(&["enter"], pick)], None);
    let click = { let on_pick = on_pick.clone(); move |item: ListItem| (on_pick.lock().expect("handler"))(item) };
    let widest = TextWidth::widest(props.items.iter().map(|i| i.label.as_str())) as u16 + props.items.iter().filter_map(|i| i.shortcut.as_ref()).map(|s| TextWidth::of(s) as u16 + 2).max().unwrap_or(0);
    element! {
        View(flex_direction: FlexDirection::Column, background_color: theme.tokens.surface, width: widest + 4) {
            List(items: props.items.clone(), selected_id: current, focused: true, on_click: click)
        }
    }
}
