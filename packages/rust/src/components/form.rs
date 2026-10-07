use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use crate::theme::TextWidth;

/// The narrowest label column a Form uses, so short labels still line up with the gallery.
pub const MIN_LABEL_WIDTH: u16 = 12;

/// The two columns every field of a Form shares. Fields register their label width while rendering;
/// the Form widens its label column to the widest one (one redraw when it changes).
#[derive(Clone)]
pub struct FormLayout {
    width: u16,
    widest: Arc<Mutex<u16>>,
    label_width: State<u16>,
}

impl FormLayout {
    /// The label column every field uses.
    pub fn label_width(&self) -> u16 {
        self.label_width.get()
    }

    /// The value column: what the label column leaves of the form's width.
    pub fn value_width(&self) -> u16 {
        self.width.saturating_sub(self.label_width() + 1).max(8)
    }

    /// Called by a field with its label: grows the column when a wider label appears.
    pub fn register(&self, label: &str) {
        // Indents move in steps of two cells, so the label column is rounded up to an even width.
        let needed = (TextWidth::of(label) as u16 + 2).max(MIN_LABEL_WIDTH).div_ceil(2) * 2;
        let mut widest = self.widest.lock().expect("form");
        if needed > *widest {
            *widest = needed;
            let mut state = self.label_width;
            state.set(needed);
        }
    }
}

pub trait UseFormLayout {
    /// The enclosing Form's columns, if any; a field registers its label and gets (label, value) widths.
    fn use_form_field(&mut self, label: &str, label_width: Option<u16>, value_width: Option<u16>) -> (u16, u16);
}

impl UseFormLayout for Hooks<'_, '_> {
    fn use_form_field(&mut self, label: &str, label_width: Option<u16>, value_width: Option<u16>) -> (u16, u16) {
        let form = self.try_use_context::<FormLayout>().map(|f| f.clone());
        if let Some(form) = &form {
            form.register(label);
        }
        let label_col = label_width.or_else(|| form.as_ref().map(|f| f.label_width())).unwrap_or(14);
        let value_col = value_width.or_else(|| form.as_ref().map(|f| f.value_width())).unwrap_or(24);
        (label_col, value_col)
    }
}

#[derive(Default, Props)]
pub struct FormProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    /// The form's width; the value column is what the label column leaves. 60 by default.
    pub width: Option<u16>,
}

/// A group of fields that share one label column and one value column, so every text field, select
/// and text area in the form is exactly as wide as its neighbours.
#[component]
pub fn Form<'a>(props: &mut FormProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let width = props.width.unwrap_or(60);
    let label_width = hooks.use_state(|| MIN_LABEL_WIDTH);
    let widest = hooks.use_const(|| Arc::new(Mutex::new(MIN_LABEL_WIDTH))).clone();
    let layout = FormLayout { width, widest, label_width };
    element! {
        ContextProvider(value: Context::owned(layout)) {
            View(flex_direction: FlexDirection::Column, width: width, flex_shrink: 0.0_f32) {
                #(props.children.iter_mut())
            }
        }
    }
}
