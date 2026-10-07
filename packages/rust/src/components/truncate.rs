use iocraft::prelude::*;

use crate::theme::TextWidth;

#[derive(Default, Props)]
pub struct TruncateProps {
    pub content: String,
    pub color: Option<Color>,
    pub weight: Weight,
    /// Push the text to the right edge (a numeric table cell).
    pub align_right: bool,
}

/// Text on one row that takes the remaining width and ends in an ellipsis when it does not fit:
/// the Rust twin of Ink's `truncate-end`. The component measures its own width after the first
/// layout; until then the text is clipped, so the first frame never overflows either.
#[component]
pub fn Truncate(props: &TruncateProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let rect = hooks.use_component_rect();
    let shown = match rect {
        Some(r) => TextWidth::truncate(&props.content, (r.right - r.left).max(0) as usize),
        None => props.content.clone(),
    };
    element! {
        View(flex_grow: 1.0_f32, flex_shrink: 1.0_f32, min_width: 0, overflow: Overflow::Hidden, justify_content: if props.align_right { JustifyContent::End } else { JustifyContent::Start }) {
            Text(content: shown, color: props.color, weight: props.weight, wrap: TextWrap::NoWrap)
        }
    }
}
