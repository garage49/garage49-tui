use iocraft::prelude::*;

use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::shell::UseTyping;
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct InputProps<'a> {
    pub value: String,
    pub on_change: HandlerMut<'static, String>,
    pub on_submit: HandlerMut<'static, String>,
    pub placeholder: Option<String>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    /// Secondary line under the text, e.g. mode and model.
    pub children: Vec<AnyElement<'a>>,
}

/// OpenCode's input: a surface behind the text with a half-block bottom edge; while focused a blue ┃
/// is painted over the surface's first column (the FocusRegion rule at field scale).
#[component]
pub fn Input<'a>(props: &mut InputProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
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
    let mut on_submit = props.on_submit.take();
    let value = props.value.clone();
    let submit = { let value = value.clone(); move || on_submit(value.clone()) };
    let backspace = { let on_change = on_change.clone(); let value = value.clone(); move || { let mut v = value.clone(); v.pop(); (on_change.lock().expect("handler"))(v) } };
    let typed = { let value = value.clone(); move |text: String| (on_change.lock().expect("handler"))(format!("{value}{text}")) };
    hooks.use_keys(props.focused, vec![Binding::new(&["enter"], submit), Binding::new(&["backspace", "delete"], backspace)], Some(Box::new(typed)));
    let focused = props.focused;
    let bar = if focused { Glyphs::BAR } else { " " };
    let has_footer = !props.children.is_empty();
    let bar_rows = if has_footer { 4 } else { 3 };
    let bar_text = (0..bar_rows).map(|_| bar).collect::<Vec<_>>().join("\n");
    let shown = if value.is_empty() { props.placeholder.clone().unwrap_or_default() } else { value.clone() };
    let content = format!("{shown}{}", if focused { Glyphs::CURSOR } else { "" });
    element! {
        View(flex_direction: FlexDirection::Column) {
            View(flex_direction: FlexDirection::Row) {
                View(width: 1, flex_shrink: 0.0_f32, background_color: t.surface) { Text(content: bar_text, color: t.accent_secondary) }
                View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, background_color: t.surface, padding_left: 2, padding_right: 2) {
                    Text(content: " ")
                    Text(content: content, color: if value.is_empty() { t.text_muted } else { t.text }, wrap: TextWrap::NoWrap)
                    Text(content: " ")
                    #(props.children.iter_mut())
                }
            }
            View(flex_direction: FlexDirection::Row, height: 1) {
                Text(content: if focused { Glyphs::EDGE_FOOT } else { Glyphs::EDGE }, color: if focused { t.accent_secondary } else { t.surface })
                View(flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden, height: 1) { Text(content: Glyphs::EDGE.repeat(400), color: t.surface, wrap: TextWrap::NoWrap) }
            }
        }
    }
}
