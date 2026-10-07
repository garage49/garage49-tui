use iocraft::prelude::*;

use crate::theme::{TextWidth, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StatusTone {
    #[default]
    Default,
    Muted,
    Accent,
    Success,
    Warning,
    Error,
}

impl StatusTone {
    fn color(self, theme: &Theme) -> Color {
        let t = &theme.tokens;
        match self {
            StatusTone::Default => t.text,
            StatusTone::Muted => t.text_muted,
            StatusTone::Accent => t.accent,
            StatusTone::Success => t.success,
            StatusTone::Warning => t.warning,
            StatusTone::Error => t.error,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StatusSegment {
    pub text: String,
    pub tone: StatusTone,
}

impl StatusSegment {
    pub fn new(text: &str, tone: StatusTone) -> Self {
        Self { text: text.into(), tone }
    }
}

#[derive(Default, Props)]
pub struct StatusLineProps {
    pub left: Vec<StatusSegment>,
    pub right: Vec<StatusSegment>,
}

/// One row on the panel surface: state segments on the left, context on the right, separated by muted dots.
#[component]
pub fn StatusLine(props: &mut StatusLineProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let (columns, _) = hooks.use_terminal_size();
    // The right side keeps its width; the left side is cut to what remains (padding 2+2, one gap).
    let right_width: usize = props.right.iter().enumerate().map(|(i, s)| TextWidth::of(&s.text) + if i > 0 { 3 } else { 0 }).sum();
    let left_width = (columns as usize).saturating_sub(4 + right_width + usize::from(!props.right.is_empty()));
    let render = |segments: &[StatusSegment], budget: usize| -> Vec<MixedTextContent> {
        let mut contents = Vec::new();
        let mut used = 0;
        for (index, segment) in segments.iter().enumerate() {
            let separator = if index > 0 { 3 } else { 0 };
            let text = TextWidth::truncate(&segment.text, budget.saturating_sub(used + separator));
            if text.is_empty() {
                break;
            }
            used += separator + TextWidth::of(&text);
            if index > 0 {
                contents.push(MixedTextContent::new(" · ").color(theme.tokens.text_muted));
            }
            contents.push(MixedTextContent::new(&text).color(segment.tone.color(&theme)));
        }
        contents
    };
    let left = render(&props.left, left_width);
    let right = render(&props.right, right_width);
    element! {
        View(flex_direction: FlexDirection::Row, height: 1, width: 100pct, background_color: theme.tokens.panel, padding_left: 2, padding_right: 2) {
            View(flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden) { MixedText(contents: left, wrap: TextWrap::NoWrap) }
            View(flex_shrink: 0.0_f32) { MixedText(contents: right, wrap: TextWrap::NoWrap) }
        }
    }
}
