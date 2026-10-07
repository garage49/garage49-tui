use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::list::{List, ListItem};
use super::overlay::Overlay;
use super::selection::Selection;
use crate::input::{Binding, UseKeys};
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct PaletteProps {
    pub title: String,
    pub items: Vec<ListItem>,
    pub on_pick: HandlerMut<'static, ListItem>,
    pub on_close: HandlerMut<'static, ()>,
}

/// Command palette: a search line followed by the filtered, sectioned list. Owns its query and cursor.
#[component]
pub fn Palette(props: &mut PaletteProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let query = hooks.use_state(String::new);
    let selected = hooks.use_state(|| None::<String>);
    let needle = query.read().trim().to_lowercase();
    let visible: Vec<ListItem> = props.items.iter().filter(|item| needle.is_empty() || item.label.to_lowercase().contains(&needle)).cloned().collect();
    let ids: Vec<String> = visible.iter().map(|item| item.id.clone()).collect();
    let current = Selection::ensure(&ids, selected.read().as_deref());
    let on_pick = Arc::new(Mutex::new(props.on_pick.take()));
    let mut on_close = props.on_close.take();
    let mover = |delta: i32| {
        let ids = ids.clone();
        let mut selected = selected;
        move || {
            let now = Selection::ensure(&ids, selected.read().as_deref());
            selected.set(Selection::move_by(&ids, now.as_deref(), delta))
        }
    };
    let pick = {
        let on_pick = on_pick.clone();
        let visible = visible.clone();
        let current = current.clone();
        move || {
            if let Some(item) = visible.iter().find(|item| Some(&item.id) == current.as_ref()) {
                (on_pick.lock().expect("handler"))(item.clone());
            }
        }
    };
    let mut query_state = query;
    let backspace = move || {
        let mut value = query_state.read().clone();
        value.pop();
        query_state.set(value);
    };
    let mut typed_state = query;
    hooks.use_keys(
        true,
        vec![
            Binding::new(&["esc"], move || on_close(())),
            Binding::new(&["up"], mover(-1)),
            Binding::new(&["down"], mover(1)),
            Binding::new(&["enter"], pick),
            Binding::new(&["backspace", "delete"], backspace),
        ],
        Some(Box::new(move |text| {
            let value = typed_state.read().clone() + &text;
            typed_state.set(value);
        })),
    );
    let click = {
        let on_pick = on_pick.clone();
        move |item: ListItem| (on_pick.lock().expect("handler"))(item)
    };
    let shown = query.read().clone();
    element! {
        Overlay(title: props.title.clone()) {
            View(padding_left: 1, padding_right: 1) {
                Text(content: format!("{}{}", if shown.is_empty() { "Search".to_string() } else { shown.clone() }, Glyphs::CURSOR), color: if shown.is_empty() { t.text_muted } else { t.text })
            }
            View(height: 1)
            List(items: visible, selected_id: current, focused: true, on_click: click)
        }
    }
}
