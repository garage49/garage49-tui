use std::time::Duration;

use iocraft::prelude::*;

use crate::theme::{Glyphs, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerKind {
    #[default]
    Dots,
    Line,
    Bounce,
}

impl SpinnerKind {
    fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerKind::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerKind::Line => &["|", "/", "-", "\\"],
            SpinnerKind::Bounce => &["⠁", "⠂", "⠄", "⠂"],
        }
    }
}

#[derive(Default, Props)]
pub struct SpinnerProps {
    pub kind: SpinnerKind,
    pub label: Option<String>,
    pub active: Option<bool>,
}

/// An in-progress marker in the accentSecondary color, with an optional muted label. Advances every 160 ms.
#[component]
pub fn Spinner(props: &mut SpinnerProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let frame = hooks.use_state(|| 0usize);
    let active = props.active.unwrap_or(true);
    hooks.use_future(async move {
        let mut frame = frame;
        loop {
            smol::Timer::after(Duration::from_millis(160)).await;
            frame.set(frame.get().wrapping_add(1));
        }
    });
    let frames = props.kind.frames();
    let glyph = if active { frames[frame.get() % frames.len()] } else { Glyphs::ON };
    let contents = match &props.label {
        Some(label) => vec![MixedTextContent::new(glyph).color(t.accent_secondary), MixedTextContent::new(format!(" {label}")).color(t.text_muted)],
        None => vec![MixedTextContent::new(glyph).color(t.accent_secondary)],
    };
    element! { MixedText(contents: contents) }
}
