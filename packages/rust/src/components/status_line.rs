use iocraft::prelude::*;

use crate::theme::Theme;

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
pub fn StatusLine(props: &mut StatusLineProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let render = |segments: &[StatusSegment]| -> Vec<MixedTextContent> {
        let mut contents = Vec::new();
        for (index, segment) in segments.iter().enumerate() {
            if index > 0 {
                contents.push(MixedTextContent::new(" · ").color(theme.tokens.text_muted));
            }
            contents.push(MixedTextContent::new(&segment.text).color(segment.tone.color(&theme)));
        }
        contents
    };
    let left = render(&props.left);
    let right = render(&props.right);
    element! {
        View(flex_direction: FlexDirection::Row, height: 1, width: 100pct, background_color: theme.tokens.panel, padding_left: 2, padding_right: 2) {
            View(flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden) { MixedText(contents: left, wrap: TextWrap::NoWrap) }
            View(flex_shrink: 0.0_f32) { MixedText(contents: right, wrap: TextWrap::NoWrap) }
        }
    }
}
