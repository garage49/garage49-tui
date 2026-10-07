use iocraft::prelude::*;

use super::tabs::{Tab, Tabs, TabsSize};
use crate::theme::{TextWidth, Theme};

#[derive(Default, Props)]
pub struct TopNavProps {
    /// The logo block at the left: bold text on the heading color; its left padding is the focus bar's column.
    pub brand: String,
    pub items: Vec<Tab>,
    pub active_id: String,
    pub focused: bool,
    pub on_change: HandlerMut<'static, Tab>,
    /// Text at the right end, e.g. the current user or version.
    pub right: Option<String>,
}

/// Web-style header, two rows: the label row on the surface color (logo block, large tabs, context on the
/// right), then a row on the screen background with a thin ▔ under the active tab.
#[component]
pub fn TopNav(props: &mut TopNavProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let brand_width = TextWidth::of(&props.brand) as u16 + 4;
    let right = props.right.clone();
    let right_width = right.as_deref().map(|r| TextWidth::of(r) as u16 + 4).unwrap_or(0);
    element! {
        View(flex_direction: FlexDirection::Row, height: 2, width: 100pct) {
            View(flex_direction: FlexDirection::Column, width: brand_width) {
                View(height: 1, background_color: t.surface) {
                    View(padding_left: 2, padding_right: 2, background_color: t.heading) { Text(content: props.brand.clone(), weight: Weight::Bold, color: t.background) }
                }
                View(height: 1)
            }
            View(flex_direction: FlexDirection::Column, width: 3) { View(height: 1, background_color: t.surface) View(height: 1) }
            Tabs(tabs: props.items.clone(), active_id: props.active_id.clone(), focused: props.focused, size: TabsSize::Large, label_surface: t.surface, on_change: props.on_change.take())
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_basis: FlexBasis::Length(0), overflow: Overflow::Hidden) { View(height: 1, background_color: t.surface) View(height: 1) }
            #(right.map(|text| element! {
                View(flex_direction: FlexDirection::Column, width: right_width) {
                    View(height: 1, background_color: t.surface, padding_left: 2, padding_right: 2) { Text(content: text, color: t.text_muted) }
                    View(height: 1)
                }
            }))
        }
    }
}
