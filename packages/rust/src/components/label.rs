use iocraft::prelude::*;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelVariant {
    #[default]
    Default,
    Muted,
    Bright,
    Heading,
    Accent,
    Success,
    Warning,
    Error,
}

impl LabelVariant {
    pub fn color(self, theme: &Theme) -> Color {
        let t = &theme.tokens;
        match self {
            LabelVariant::Default => t.text,
            LabelVariant::Muted => t.text_muted,
            LabelVariant::Bright => t.text_bright,
            LabelVariant::Heading => t.heading,
            LabelVariant::Accent => t.accent,
            LabelVariant::Success => t.success,
            LabelVariant::Warning => t.warning,
            LabelVariant::Error => t.error,
        }
    }
}

#[derive(Default, Props)]
pub struct LabelProps {
    pub content: String,
    pub variant: LabelVariant,
    pub wrap: Option<TextWrap>,
}

/// Text in one of the theme's semantic roles. Headings and bright text are bold.
#[component]
pub fn Label(props: &mut LabelProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let bold = matches!(props.variant, LabelVariant::Heading | LabelVariant::Bright);
    let weight = if bold { Weight::Bold } else { Weight::Normal };
    let wrap = props.wrap.unwrap_or(TextWrap::Wrap);
    element! {
        Text(content: props.content.clone(), color: props.variant.color(&theme), weight: weight, wrap: wrap)
    }
}
