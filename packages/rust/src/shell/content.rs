use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct ContentProps<'a> {
    pub children: Vec<AnyElement<'a>>,
}

/// The row between the navigation and the bottom bars: a Sidebar and a Main side by side.
#[component]
pub fn Content<'a>(props: &mut ContentProps<'a>) -> impl Into<AnyElement<'a>> {
    element! {
        View(flex_direction: FlexDirection::Row, flex_grow: 1.0_f32) {
            #(props.children.iter_mut())
        }
    }
}
