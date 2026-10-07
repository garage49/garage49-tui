use iocraft::prelude::*;

use super::field_bar::FieldBar;
use super::form::UseFormLayout;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct ToggleProps {
    pub label: String,
    pub value: bool,
    pub on_change: HandlerMut<'static, bool>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    pub label_width: Option<u16>,
}

/// A boolean form field: label, then the on glyph + "on" (success color) or the off glyph + "off" (muted); space or click toggles.
#[component]
pub fn Toggle(props: &mut ToggleProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let (label_col, _) = hooks.use_form_field(&props.label, props.label_width, None);
    let on_change = std::sync::Arc::new(std::sync::Mutex::new(props.on_change.take()));
    let mut on_focus = props.on_focus.take();
    let value = props.value;
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let on_change = on_change.clone();
        hooks.use_mouse(allowed, move |event| {
            if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                on_focus(());
                (on_change.lock().expect("handler"))(!value);
            }
        });
    }
    hooks.use_keys(props.focused, vec![Binding::new(&["space", "enter"], move || (on_change.lock().expect("handler"))(!value))], None);
    let focused = props.focused;
    element! {
        View(flex_direction: FlexDirection::Row, height: 1) {
            View(width: label_col, flex_shrink: 0.0_f32) { Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted }) }
            FieldBar(focused: focused)
            View(background_color: t.surface, padding_left: 1, padding_right: 1) {
                Text(content: if value { format!("{} on ", Glyphs::ON) } else { format!("{} off", Glyphs::OFF) }, weight: if value { Weight::Bold } else { Weight::Normal }, color: if value { t.success } else { t.text_muted })
            }
        }
    }
}
