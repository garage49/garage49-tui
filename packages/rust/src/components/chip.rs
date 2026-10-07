use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipTone {
    #[default]
    Default,
    Accent,
    Success,
    Warning,
    Error,
}

#[derive(Default, Props)]
pub struct ChipProps {
    pub label: String,
    /// Colors the label; the pill stays on the raised surface so tone reads as a state, not a cursor.
    pub tone: ChipTone,
    /// The cursor is on this chip: filled with the accent color.
    pub selected: bool,
    /// Shows a × and calls on_remove when it is clicked.
    pub removable: bool,
    pub on_remove: HandlerMut<'static, ()>,
    pub on_press: HandlerMut<'static, ()>,
}

/// A small pill on the raised surface: a tag, a filter, a status. Click selects; × removes.
#[component]
pub fn Chip(props: &mut ChipProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let tone = match props.tone {
        ChipTone::Default => t.text,
        ChipTone::Accent => t.accent,
        ChipTone::Success => t.success,
        ChipTone::Warning => t.warning,
        ChipTone::Error => t.error,
    };
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    let label_width = crate::theme::TextWidth::of(&props.label) as i32;
    let removable = props.removable;
    {
        let mut on_remove = props.on_remove.take();
        let mut on_press = props.on_press.take();
        hooks.use_mouse(allowed, move |event| {
            if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                return;
            }
            if removable && event.local_x >= label_width + 2 {
                on_remove(());
            } else {
                on_press(());
            }
        });
    }
    let selected = props.selected;
    element! {
        View(padding_left: 1, padding_right: 1, margin_right: 1, background_color: if selected { t.selection_background } else { t.surface_raised }, flex_direction: FlexDirection::Row) {
            Text(content: props.label.clone(), weight: if selected { Weight::Bold } else { Weight::Normal }, color: if selected { t.selection_text } else { tone })
            #(if removable { Some(element! { Text(content: " ×", color: if selected { t.selection_text } else { t.text_muted }) }) } else { None })
        }
    }
}
