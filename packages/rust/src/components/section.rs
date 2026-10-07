use iocraft::prelude::*;

use super::label::{Label, LabelVariant};

#[derive(Default, Props)]
pub struct SectionProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub title: Option<String>,
}

/// One titled area of a page. The title is a heading; the body starts right under it; one blank row
/// separates it from the next section. Pages are composed of sections, never of hand-padded views.
#[component]
pub fn Section<'a>(props: &mut SectionProps<'a>) -> impl Into<AnyElement<'a>> {
    let title = props.title.clone();
    element! {
        View(flex_direction: FlexDirection::Column, margin_bottom: 1, flex_shrink: 0.0_f32) {
            #(title.map(|t| element! { Label(content: t, variant: LabelVariant::Heading) }))
            #(props.children.iter_mut())
        }
    }
}
