use iocraft::prelude::*;

use super::focus_region::RegionSurface;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OverlayVariant {
    #[default]
    Default,
    Error,
}

#[derive(Default, Props)]
pub struct OverlayProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub title: String,
    pub width: Option<u16>,
    pub variant: OverlayVariant,
}

/// A centered panel without a border: bold title at the left, "esc" muted at the right.
#[component]
pub fn Overlay<'a>(props: &mut OverlayProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let title_color = if props.variant == OverlayVariant::Error { t.error } else { t.text };
    element! {
        View(flex_direction: FlexDirection::Column, width: props.width.unwrap_or(60), background_color: t.panel, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1) {
            View(flex_direction: FlexDirection::Row) {
                Text(content: props.title.clone(), weight: Weight::Bold, color: title_color)
                View(flex_grow: 1.0_f32)
                Text(content: "esc", color: t.text_muted)
            }
            View(height: 1)
            ContextProvider(value: Context::owned(RegionSurface::Panel)) {
                #(props.children.iter_mut())
            }
        }
    }
}
