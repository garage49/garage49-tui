use iocraft::prelude::*;

use crate::theme::Theme;

#[derive(Default, Props)]
pub struct SplitProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    /// Stack the panes instead of placing them side by side.
    pub stacked: bool,
}

/// Sub-panes side by side (or stacked): each child becomes a pane on its own surface — alternating
/// background and panel so the edge is visible without a line — with the standard padding (2 columns,
/// 1 row) and one cell between panes. Each pane takes an equal share of the space.
#[component]
pub fn Split<'a>(props: &mut SplitProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let direction = if props.stacked { FlexDirection::Column } else { FlexDirection::Row };
    element! {
        View(flex_direction: direction, flex_grow: 1.0_f32, gap: 1, min_height: 0) {
            #(props.children.iter_mut().enumerate().map(|(index, child)| element! {
                View(key: index, flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), min_height: 0, overflow: Overflow::Hidden,
                     padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1,
                     background_color: if index % 2 == 1 { t.panel } else { t.background }) {
                    #(std::iter::once(child))
                }
            }))
        }
    }
}
