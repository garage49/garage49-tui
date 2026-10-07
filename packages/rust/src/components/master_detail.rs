use iocraft::prelude::*;

use super::list::{List, ListItem};
use super::focus_region::RegionSurface;
use super::list_layout::{ListLayout, ListRow};
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct MasterDetailProps<'a> {
    pub items: Vec<ListItem>,
    pub selected_id: Option<String>,
    pub focused: bool,
    /// A left click on a row.
    pub on_click: HandlerMut<'static, ListItem>,
    /// The wheel moving the selection to a neighbouring row.
    pub on_select: HandlerMut<'static, ListItem>,
    /// The detail of the selected row.
    pub children: Vec<AnyElement<'a>>,
}

/// A list and the detail of its selected row as one control inside a section: the list takes the
/// left third, the detail the rest on the surface token, and the selected row's fill continues across
/// the one-cell gap into the detail, so the two read as connected rather than as two sections.
#[component]
pub fn MasterDetail<'a>(props: &mut MasterDetailProps<'a>, hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let t = hooks.use_context::<Theme>().tokens;
    let selected = props.selected_id.clone();
    let row = ListLayout::rows(&props.items)
        .iter()
        .position(|r| matches!(r, ListRow::Item(item) if Some(item.id.as_str()) == selected.as_deref()));
    let fill = if props.focused { t.selection_background } else { t.surface_raised };
    let (items, focused) = (props.items.clone(), props.focused);
    let (on_click, on_select) = (props.on_click.take(), props.on_select.take());
    element! {
        View(flex_direction: FlexDirection::Row, flex_grow: 1.0_f32, min_height: 0) {
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), min_height: 0) {
                List(items: items, selected_id: selected, focused: focused, on_click: on_click, on_select: on_select)
            }
            View(flex_direction: FlexDirection::Column, width: 1, flex_shrink: 0.0_f32, padding_top: row.unwrap_or(0) as u32) {
                #(row.map(|_| element! { View(height: 1, background_color: fill) }))
            }
            View(flex_direction: FlexDirection::Column, flex_grow: 2.0_f32, flex_basis: FlexBasis::Length(0), min_height: 0, background_color: t.surface,
                 padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1) {
                ContextProvider(value: Context::owned(RegionSurface::Surface)) {
                    #(props.children.iter_mut())
                }
            }
        }
    }
}
