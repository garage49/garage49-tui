use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::dropdown::Dropdown;
use super::field_bar::FieldBar;
use super::form::UseFormLayout;
use super::list::ListItem;
use super::screen::OverlayHandle;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct SelectProps {
    pub label: String,
    pub options: Vec<ListItem>,
    pub value: String,
    pub on_change: HandlerMut<'static, ListItem>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    pub label_width: Option<u16>,
    pub width: Option<u16>,
}

/// A select box: the current option with a ▾ on a surface. Enter, space or a click opens a dropdown; ←→ cycle without opening.
/// The root is a View: iocraft delivers local mouse events only to components whose root node is a View.
#[component]
pub fn Select(props: &mut SelectProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let overlay = *hooks.use_context::<OverlayHandle>();
    let rect = hooks.use_component_rect();
    let (label_width, value_width) = hooks.use_form_field(&props.label, props.label_width, props.width);
    let options = props.options.clone();
    let value = props.value.clone();
    let index = options.iter().position(|o| o.id == value).unwrap_or(0);
    let on_change = Arc::new(Mutex::new(props.on_change.take()));
    let open = {
        let options = options.clone();
        let value = value.clone();
        let on_change = on_change.clone();
        let (top, left) = rect.map(|r| (r.top + 1, r.left + label_width as i32)).unwrap_or((0, 0));
        move || {
            let options = options.clone();
            let value = value.clone();
            let on_change = on_change.clone();
            overlay.show_at(
                Arc::new(move || {
                    let on_change = on_change.clone();
                    element! {
                        Dropdown(items: options.clone(), initial_id: value.clone(), on_close: move |_| overlay.hide(), on_pick: move |item: ListItem| { overlay.hide(); (on_change.lock().expect("handler"))(item); })
                    }
                    .into_any()
                }),
                top,
                left,
            );
        }
    };
    let cycle = |delta: i32| { let options = options.clone(); let on_change = on_change.clone(); move || { let len = options.len() as i32; if len == 0 { return; } let next = ((index as i32 + delta) % len + len) % len; (on_change.lock().expect("handler"))(options[next as usize].clone()); } };
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let mut on_focus = props.on_focus.take();
        let open = open.clone();
        hooks.use_mouse(allowed, move |event| {
            if matches!(event.kind, MouseEventKind::Down(_)) {
                on_focus(());
                open();
            }
        });
    }
    hooks.use_keys(props.focused, vec![Binding::new(&["enter", "space"], open.clone()), Binding::new(&["left"], cycle(-1)), Binding::new(&["right"], cycle(1))], None);
    let focused = props.focused;
    let current = options.iter().find(|o| o.id == value).map(|o| o.label.clone()).unwrap_or_default();
    element! {
        View(flex_direction: FlexDirection::Row, height: 1) {
            View(width: label_width, flex_shrink: 0.0_f32) { Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted }) }
            FieldBar(focused: focused)
            View(width: value_width, background_color: t.surface, padding_left: 1, padding_right: 1, flex_direction: FlexDirection::Row) {
                View(flex_grow: 1.0_f32, overflow: Overflow::Hidden) { Text(content: current, color: t.text, wrap: TextWrap::NoWrap) }
                Text(content: " ▾", color: if focused { t.text } else { t.text_muted })
            }
        }
    }
}
