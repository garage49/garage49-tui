use iocraft::prelude::*;

use super::field_bar::FieldBar;
use super::focus_region::RegionSurface;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct CheckboxProps {
    pub label: String,
    pub checked: bool,
    pub on_change: HandlerMut<'static, bool>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
}

/// A check box on the screen background: ☑ in the success color when checked, ☐ muted otherwise. Space or click toggles.
#[component]
pub fn Checkbox(props: &mut CheckboxProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let on_change = std::sync::Arc::new(std::sync::Mutex::new(props.on_change.take()));
    let mut on_focus = props.on_focus.take();
    let checked = props.checked;
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let on_change = on_change.clone();
        hooks.use_mouse(allowed, move |event| {
            if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                on_focus(());
                (on_change.lock().expect("handler"))(!checked);
            }
        });
    }
    hooks.use_keys(props.focused, vec![Binding::new(&["space", "enter"], move || (on_change.lock().expect("handler"))(!checked))], None);
    let focused = props.focused;
    element! {
        View(flex_direction: FlexDirection::Row, height: 1) {
            FieldBar(focused: focused, surface: RegionSurface::Background)
            Text(content: format!("{} ", if checked { Glyphs::CHECKED } else { Glyphs::UNCHECKED }), weight: Weight::Bold, color: if checked { t.success } else { t.text_muted })
            Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted })
        }
    }
}
