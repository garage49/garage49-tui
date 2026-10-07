use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::button::Button;
use super::label::Label;
use super::overlay::{Overlay, OverlayVariant};
use crate::input::{Binding, UseKeys};

#[derive(Default, Props)]
pub struct ConfirmDialogProps {
    pub title: String,
    pub message: String,
    pub confirm_label: Option<String>,
    pub cancel_label: Option<String>,
    /// Marks the confirming action as destructive: the dialog title turns to the error color.
    pub danger: bool,
    pub on_confirm: HandlerMut<'static, ()>,
    pub on_cancel: HandlerMut<'static, ()>,
}

/// A yes/no dialog: message, then two buttons; the chosen button is filled with the accent color.
#[component]
pub fn ConfirmDialog(props: &mut ConfirmDialogProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let confirm_selected = hooks.use_state(|| false);
    let on_confirm = Arc::new(Mutex::new(props.on_confirm.take()));
    let on_cancel = Arc::new(Mutex::new(props.on_cancel.take()));
    let confirm = {
        let on_confirm = on_confirm.clone();
        move || (on_confirm.lock().expect("handler"))(())
    };
    let cancel = {
        let on_cancel = on_cancel.clone();
        move || (on_cancel.lock().expect("handler"))(())
    };
    let selected = confirm_selected.get();
    let mut toggle_state = confirm_selected;
    let enter = {
        let confirm = confirm.clone();
        let cancel = cancel.clone();
        move || if selected { confirm() } else { cancel() }
    };
    hooks.use_keys(
        true,
        vec![
            Binding::new(&["esc", "n"], cancel.clone()),
            Binding::new(&["y"], confirm.clone()),
            Binding::new(&["left", "right", "tab"], move || toggle_state.set(!selected)),
            Binding::new(&["enter"], enter),
        ],
        None,
    );
    let variant = if props.danger { OverlayVariant::Error } else { OverlayVariant::Default };
    element! {
        Overlay(title: props.title.clone(), width: 50u16, variant: variant) {
            View(padding_left: 1, padding_right: 1) { Label(content: props.message.clone()) }
            View(height: 1)
            View(flex_direction: FlexDirection::Row, justify_content: JustifyContent::End) {
                Button(label: props.cancel_label.clone().unwrap_or_else(|| "Cancel".into()), selected: !selected, on_press: move |_| cancel())
                Button(label: props.confirm_label.clone().unwrap_or_else(|| "OK".into()), selected: selected, on_press: move |_| confirm())
            }
        }
    }
}
