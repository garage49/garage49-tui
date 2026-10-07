use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};

/// Whether a text field is editing: it is the page's current field (`focused`) and no click has landed
/// outside it since. A click outside ends editing (cursor and bar gone, keys back to the app); a click on
/// the field (`resume`) or the field becoming current again starts it anew.
pub trait UseEditing {
    fn use_editing(&mut self, focused: bool) -> Editing;
}

#[derive(Clone, Copy)]
pub struct Editing {
    pub editing: bool,
    blurred: State<bool>,
}

impl Editing {
    pub fn resume(&self) {
        let mut blurred = self.blurred;
        blurred.set(false);
    }
}

impl UseEditing for Hooks<'_, '_> {
    fn use_editing(&mut self, focused: bool) -> Editing {
        let blurred = self.use_state(|| false);
        if !focused && blurred.get() {
            let mut blurred = blurred;
            blurred.set(false);
        }
        let allowed = self.mouse_allowed(MouseLayer::Screen);
        self.use_mouse_outside(allowed, move || {
            if focused {
                let mut blurred = blurred;
                blurred.set(true);
            }
        });
        Editing { editing: focused && !blurred.get(), blurred }
    }
}
