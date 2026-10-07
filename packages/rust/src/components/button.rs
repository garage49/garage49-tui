use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct ButtonProps {
    pub label: String,
    pub selected: bool,
    pub on_press: HandlerMut<'static, ()>,
}

/// A clickable label on a raised surface; the selected button is filled with the accent color.
#[component]
pub fn Button(props: &mut ButtonProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    let mut on_press = props.on_press.take();
    hooks.use_mouse(allowed, move |event| {
        if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            on_press(());
        }
    });
    let selected = props.selected;
    element! {
        View(background_color: if selected { t.selection_background } else { t.surface_raised }, padding_left: 2, padding_right: 2, margin_left: 2) {
            Text(content: props.label.clone(), weight: if selected { Weight::Bold } else { Weight::Normal }, color: if selected { t.selection_text } else { t.text })
        }
    }
}
