use iocraft::prelude::*;

use super::focus::{FocusState, UseFocusRegion};
use crate::components::{FocusRegion, Label, LabelVariant, OverlayHandle};
use crate::input::{MouseLayer, UseMouse};

#[derive(Default, Props)]
pub struct MainProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub title: Option<String>,
}

/// Whether the enclosing Main owns the keyboard; pages read it through `use_context::<MainFocus>()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MainFocus(pub bool);

/// The App's page area: a focus region that takes the remaining width. Enter is the page's own key here.
/// The root is a View: iocraft delivers local mouse events only to components whose root node is a View.
#[component]
pub fn Main<'a>(props: &mut MainProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let (id, focused) = hooks.use_focus_region(false);
    let overlay_open = hooks.use_context::<OverlayHandle>().is_open();
    let active = focused && !overlay_open;
    let focus = hooks.use_context::<FocusState>().clone();
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    hooks.use_mouse(allowed, move |event| {
        if matches!(event.kind, MouseEventKind::Down(_)) {
            focus.focus(id);
        }
    });
    let title = props.title.clone();
    element! {
        View(flex_direction: FlexDirection::Row, flex_grow: 1.0_f32) {
        FocusRegion(focused: active, grow: true) {
            ContextProvider(value: Context::owned(MainFocus(active))) {
                View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, padding_left: 2, padding_right: 2, padding_top: 1, padding_bottom: 1, overflow: Overflow::Hidden) {
                    #(title.map(|title| element! {
                        View(flex_direction: FlexDirection::Column) {
                            Label(content: title, variant: LabelVariant::Bright)
                            View(height: 1)
                        }
                    }))
                    View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
                        #(props.children.iter_mut())
                    }
                }
            }
        }
        }
    }
}
