use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct SplitProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    /// Stack the sections instead of placing them side by side.
    pub stacked: bool,
    /// Fill the remaining height (when its sections grow).
    pub grow: bool,
}

/// Sections side by side (or stacked): each child is a Section and takes an equal share of the space,
/// one cell apart. The split draws nothing of its own — no surface, no padding — so the structure a
/// reader sees is only ever "sections": the header bars and panel blocks are the Sections' own.
#[component]
pub fn Split<'a>(props: &mut SplitProps<'a>) -> impl Into<AnyElement<'a>> {
    let (direction, gap) = if props.stacked { (FlexDirection::Column, 0) } else { (FlexDirection::Row, 1) };
    let grow = if props.grow { 1.0_f32 } else { 0.0_f32 };
    element! {
        View(flex_direction: direction, flex_grow: grow, flex_shrink: grow, gap: gap, min_height: 0) {
            #(props.children.iter_mut().enumerate().map(|(index, child)| element! {
                View(key: index, flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), min_height: 0, overflow: Overflow::Hidden) {
                    #(std::iter::once(child))
                }
            }))
        }
    }
}
