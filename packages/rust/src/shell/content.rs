use iocraft::prelude::*;

use super::fit::ScreenFit;

#[derive(Default, Props)]
pub struct ContentProps<'a> {
    pub children: Vec<AnyElement<'a>>,
}

/// The row between the navigation and the bottom bars: a Sidebar and a Main side by side.
/// In a narrow terminal (ScreenFit) only the first child, the Sidebar, is kept.
#[component]
pub fn Content<'a>(props: &mut ContentProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let (columns, rows) = hooks.use_terminal_size();
    let fit = ScreenFit::decide(columns, rows, props.children.len() > 1);
    let keep = if fit.main { props.children.len() } else { 1 };
    element! {
        View(flex_direction: FlexDirection::Row, flex_grow: 1.0_f32, flex_shrink: 1.0_f32, min_height: 0, overflow: Overflow::Hidden) {
            #(props.children.iter_mut().take(keep))
        }
    }
}
