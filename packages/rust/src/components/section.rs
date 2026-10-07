use iocraft::prelude::*;

use super::focus_region::RegionSurface;
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct SectionProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub title: String,
    /// Fill the remaining height (a log, a long list).
    pub grow: bool,
}

/// One titled area of a page, drawn as a block on the panel surface with the standard padding
/// (2 columns, 1 row). The title is a header bar: bold bright text on the raised surface across the
/// block, so it outranks the group headings inside (list sections, form groups). One blank row of
/// page background separates sections. Sections never nest; nothing is placed above a section title.
/// Every section has a title: a block without a header bar is an Intro, not a section.
#[component]
pub fn Section<'a>(props: &mut SectionProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let title = props.title.clone();
    let (grow, shrink) = if props.grow { (1.0_f32, 1.0_f32) } else { (0.0_f32, 0.0_f32) };
    let t = theme.tokens;
    element! {
        View(flex_direction: FlexDirection::Column, margin_bottom: 1, flex_grow: grow, flex_shrink: shrink, min_height: 0, overflow: Overflow::Hidden, background_color: t.panel) {
            View(height: 1, padding_left: 2, padding_right: 2, background_color: t.surface_raised) {
                Text(content: title, weight: Weight::Bold, color: t.text_bright)
            }
            View(flex_direction: FlexDirection::Column, flex_grow: grow, min_height: 0, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1) {
                ContextProvider(value: Context::owned(RegionSurface::Panel)) {
                    #(props.children.iter_mut())
                }
            }
        }
    }
}
