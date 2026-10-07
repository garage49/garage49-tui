use iocraft::prelude::*;

use super::overlay::Overlay;
use crate::input::{Binding, UseKeys};
use crate::theme::{TextWidth, Theme};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HelpEntry {
    pub keys: String,
    pub action: String,
}

impl HelpEntry {
    pub fn new(keys: &str, action: &str) -> Self {
        Self { keys: keys.into(), action: action.into() }
    }
}

#[derive(Default, Props)]
pub struct HelpOverlayProps {
    pub entries: Vec<HelpEntry>,
    pub on_close: HandlerMut<'static, ()>,
}

/// The `?` help overlay: one line per key binding.
#[component]
pub fn HelpOverlay(props: &mut HelpOverlayProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let mut on_close = props.on_close.take();
    hooks.use_keys(true, vec![Binding::new(&["esc", "?", "q"], move || on_close(()))], None);
    let key_width = TextWidth::widest(props.entries.iter().map(|e| e.keys.as_str())) as u16 + 2;
    element! {
        Overlay(title: "Help") {
            View(flex_direction: FlexDirection::Column) {
                #(props.entries.iter().enumerate().map(|(index, entry)| element! {
                    View(key: index, flex_direction: FlexDirection::Row) {
                        View(width: key_width, flex_shrink: 0.0_f32) { Text(content: entry.keys.clone(), color: t.text) }
                        Text(content: entry.action.clone(), color: t.text_muted, wrap: TextWrap::NoWrap)
                    }
                }))
            }
        }
    }
}
