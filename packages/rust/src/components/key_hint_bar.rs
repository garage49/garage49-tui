use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};
use crate::theme::{TextWidth, Theme};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct KeyHint {
    pub key: String,
    pub label: String,
}

impl KeyHint {
    pub fn new(key: &str, label: &str) -> Self {
        Self { key: key.into(), label: label.into() }
    }
}

#[derive(Default, Props)]
pub struct KeyHintBarProps {
    pub hints: Vec<KeyHint>,
    pub left: String,
    pub on_press: HandlerMut<'static, KeyHint>,
}

/// Bottom line: muted context on the left, "key (bright) + label (muted)" pairs on the right. A click on a pair triggers it.
#[component]
pub fn KeyHintBar(props: &mut KeyHintBarProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let widths: Vec<i32> = props.hints.iter().map(|h| 2 + TextWidth::of(&h.key) as i32 + 1 + TextWidth::of(&h.label) as i32).collect();
    let total: i32 = widths.iter().sum();
    let rect = hooks.use_component_rect();
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let hints = props.hints.clone();
        let widths = widths.clone();
        let mut on_press = props.on_press.take();
        let width = rect.map(|r| r.right - r.left).unwrap_or(0);
        hooks.use_mouse(allowed, move |event| {
            if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                return;
            }
            let mut x = event.local_x - (width - total);
            for (index, w) in widths.iter().enumerate() {
                if x >= 0 && x < *w {
                    if let Some(hint) = hints.get(index) {
                        on_press(hint.clone());
                    }
                    return;
                }
                x -= *w;
            }
        });
    }
    let pairs: Vec<MixedTextContent> = props
        .hints
        .iter()
        .flat_map(|hint| {
            vec![
                MixedTextContent::new(format!("  {} ", hint.key)).color(t.text),
                MixedTextContent::new(&hint.label).color(t.text_muted),
            ]
        })
        .collect();
    element! {
        View(flex_direction: FlexDirection::Row, height: 1, width: 100pct) {
            View(flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden) { Text(content: props.left.clone(), color: t.text_muted, wrap: TextWrap::NoWrap) }
            View(flex_shrink: 0.0_f32) { MixedText(contents: pairs, wrap: TextWrap::NoWrap) }
        }
    }
}
