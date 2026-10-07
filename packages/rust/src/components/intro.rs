use iocraft::prelude::*;

use super::focus_region::RegionSurface;
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct IntroProps<'a> {
    pub title: String,
    pub children: Vec<AnyElement<'a>>,
}

/// A page's introduction: a block on the panel surface with no header bar, at the top of the page
/// before its sections, holding one bright title line and a short muted explanation. It is not a
/// section (nothing is listed or edited in it), so it has no header bar; at most one per page.
#[component]
pub fn Intro<'a>(props: &mut IntroProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let t = hooks.use_context::<Theme>().tokens;
    element! {
        View(flex_direction: FlexDirection::Column, margin_bottom: 1, flex_shrink: 0.0_f32, background_color: t.panel, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1) {
            Text(content: props.title.clone(), color: t.text_bright)
            ContextProvider(value: Context::owned(RegionSurface::Panel)) {
                #(props.children.iter_mut())
            }
        }
    }
}
