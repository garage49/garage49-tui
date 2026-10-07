use iocraft::prelude::*;

use super::button::Button;
use super::label::Label;
use super::overlay::{Overlay, OverlayVariant};
use crate::input::{Binding, UseKeys};

#[derive(Default, Props)]
pub struct MessageDialogProps {
    pub title: String,
    pub message: String,
    pub variant: OverlayVariant,
    pub on_close: HandlerMut<'static, ()>,
}

/// A one-button dialog for information and errors.
#[component]
pub fn MessageDialog(props: &mut MessageDialogProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let on_close = std::sync::Arc::new(std::sync::Mutex::new(props.on_close.take()));
    let close = { let on_close = on_close.clone(); move || (on_close.lock().expect("handler"))(()) };
    hooks.use_keys(true, vec![Binding::new(&["esc", "enter"], close.clone())], None);
    element! {
        Overlay(title: props.title.clone(), width: 50u16, variant: props.variant) {
            View { Label(content: props.message.clone()) }
            View(height: 1)
            View(flex_direction: FlexDirection::Row, justify_content: JustifyContent::End) {
                Button(label: "OK", selected: true, on_press: move |_| close())
            }
        }
    }
}
