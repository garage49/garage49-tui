use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::field_bar::FieldBar;
use super::focus_region::RegionSurface;
use super::list::ListItem;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct RadioGroupProps {
    pub label: String,
    pub options: Vec<ListItem>,
    pub value: String,
    pub on_change: HandlerMut<'static, ListItem>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    pub label_width: Option<u16>,
    /// ↑ on the first option or ↓ on the last: hand the focus to the neighbouring field (-1 / 1).
    pub on_leave: HandlerMut<'static, i32>,
}

/// One field with one row per option: the on glyph for the chosen one in the success color, the off glyph muted otherwise.
/// ↑↓ move the cursor (the accent-filled row); space or enter chooses; a click chooses directly.
#[component]
pub fn RadioGroup(props: &mut RadioGroupProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let options = props.options.clone();
    let chosen = options.iter().position(|o| o.id == props.value).unwrap_or(0);
    let cursor = hooks.use_state(move || chosen);
    let on_change = Arc::new(Mutex::new(props.on_change.take()));
    let on_leave = Arc::new(Mutex::new(props.on_leave.take()));
    let choose = { let options = options.clone(); let on_change = on_change.clone(); move |index: usize| { if let Some(option) = options.get(index) { (on_change.lock().expect("handler"))(option.clone()); } } };
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let mut on_focus = props.on_focus.take();
        let choose = choose.clone();
        let mut cursor = cursor;
        let len = options.len();
        hooks.use_mouse(allowed, move |event| {
            if matches!(event.kind, MouseEventKind::Down(_)) {
                on_focus(());
                let row = event.local_y.max(0) as usize;
                if row < len { cursor.set(row); choose(row); }
            }
        });
    }
    let current = cursor.get();
    let len = options.len();
    let up = { let on_leave = on_leave.clone(); let mut cursor = cursor; move || { let now = cursor.get(); if now == 0 { (on_leave.lock().expect("handler"))(-1) } else { cursor.set(now - 1) } } };
    let down = { let on_leave = on_leave.clone(); let mut cursor = cursor; move || { let now = cursor.get(); if now + 1 >= len { (on_leave.lock().expect("handler"))(1) } else { cursor.set(now + 1) } } };
    let pick = { let choose = choose.clone(); move || choose(current) };
    hooks.use_keys(props.focused, vec![Binding::new(&["up", "k"], up), Binding::new(&["down", "j"], down), Binding::new(&["space", "enter"], pick)], None);
    let focused = props.focused;
    let value = props.value.clone();
    element! {
        View(flex_direction: FlexDirection::Row, height: len as u16) {
            View(width: props.label_width.unwrap_or(14), flex_shrink: 0.0_f32) { Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted }) }
            FieldBar(focused: focused, surface: RegionSurface::Background, rows: len as u16)
            View(flex_direction: FlexDirection::Column) {
                #(options.iter().enumerate().map(|(index, option)| {
                    let is_chosen = option.id == value;
                    let highlighted = focused && index == current;
                    let fill = if highlighted { Some(t.selection_background) } else { None };
                    let mark = if highlighted { t.selection_text } else if is_chosen { t.success } else { t.text_muted };
                    let label_color = if highlighted { t.selection_text } else if is_chosen || focused { t.text } else { t.text_muted };
                    element! {
                        View(key: index, flex_direction: FlexDirection::Row, background_color: fill, padding_left: 1, padding_right: 1) {
                            Text(content: format!("{} ", if is_chosen { Glyphs::ON } else { Glyphs::OFF }), weight: Weight::Bold, color: mark)
                            Text(content: option.label.clone(), weight: if highlighted { Weight::Bold } else { Weight::Normal }, color: label_color)
                        }
                    }
                }))
            }
        }
    }
}
