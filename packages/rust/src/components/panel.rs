use iocraft::prelude::*;

use crate::theme::Theme;

#[derive(Default, Props)]
pub struct PanelProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub raised: bool,
    pub width: Option<u16>,
    pub grow: bool,
}

/// A region separated from its neighbours by background color only — no border lines.
#[component]
pub fn Panel<'a>(props: &mut PanelProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let background = if props.raised { theme.tokens.surface } else { theme.tokens.panel };
    let grow = if props.grow { 1.0_f32 } else { 0.0_f32 };
    match props.width {
        Some(width) => element! {
            View(flex_direction: FlexDirection::Column, background_color: background, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1, width: width, flex_shrink: 0.0_f32, flex_grow: grow) {
                #(props.children.iter_mut())
            }
        }
        .into_any(),
        None => element! {
            View(flex_direction: FlexDirection::Column, background_color: background, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1, flex_grow: grow) {
                #(props.children.iter_mut())
            }
        }
        .into_any(),
    }
}
