use iocraft::prelude::*;

use super::focus::RegionId;

/// Tracks whether a text field is being edited, so single-letter app keys (q, ?, t, m) stay out of the way.
#[derive(Clone, Copy)]
pub struct TypingState {
    pub editing: State<Option<RegionId>>,
}

impl TypingState {
    pub fn typing(&self) -> bool {
        self.editing.get().is_some()
    }
}

pub trait UseTyping {
    /// Called by text fields every render: while `editing`, the app treats printable keys as text.
    fn use_typing(&mut self, editing: bool);
}

impl UseTyping for Hooks<'_, '_> {
    fn use_typing(&mut self, editing: bool) {
        let id = self.use_const(super::focus_next_field_id);
        let Some(state) = self.try_use_context::<TypingState>().map(|s| *s) else { return };
        let current = state.editing.get();
        let mut editing_state = state.editing;
        if editing && current != Some(id) {
            editing_state.set(Some(id));
        } else if !editing && current == Some(id) {
            editing_state.set(None);
        }
    }
}
