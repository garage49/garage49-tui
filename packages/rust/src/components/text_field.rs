use iocraft::prelude::*;

use super::field_bar::FieldBar;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::shell::UseTyping;
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct TextFieldProps {
    pub label: String,
    pub value: String,
    pub on_change: HandlerMut<'static, String>,
    pub placeholder: Option<String>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    pub label_width: Option<u16>,
}

/// A one-line form field: label on the left, the value on a surface; the focused field gets the accent bar.
#[component]
pub fn TextField(props: &mut TextFieldProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    hooks.use_typing(props.focused);
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    let mut on_focus = props.on_focus.take();
    hooks.use_mouse(allowed, move |event| {
        if matches!(event.kind, MouseEventKind::Down(_)) {
            on_focus(());
        }
    });
    let on_change = std::sync::Arc::new(std::sync::Mutex::new(props.on_change.take()));
    let value = props.value.clone();
    let backspace = {
        let on_change = on_change.clone();
        let value = value.clone();
        move || {
            let mut next = value.clone();
            next.pop();
            (on_change.lock().expect("handler"))(next);
        }
    };
    let typed = {
        let value = value.clone();
        move |text: String| (on_change.lock().expect("handler"))(format!("{value}{text}"))
    };
    hooks.use_keys(props.focused, vec![Binding::new(&["backspace", "delete"], backspace)], Some(Box::new(typed)));
    let focused = props.focused;
    let shown = if value.is_empty() { props.placeholder.clone().unwrap_or_default() } else { value.clone() };
    let content = format!("{shown}{}", if focused { Glyphs::CURSOR } else { "" });
    element! {
        View(flex_direction: FlexDirection::Row, height: 1) {
            View(width: props.label_width.unwrap_or(14), flex_shrink: 0.0_f32) { Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted }) }
            FieldBar(focused: focused)
            View(flex_grow: 1.0_f32, background_color: t.surface, padding_left: 1, padding_right: 1) {
                Text(content: content, color: if value.is_empty() { t.text_muted } else { t.text }, wrap: TextWrap::NoWrap)
            }
        }
    }
}
