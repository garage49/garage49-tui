use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::focus::RegionId;

/// Tracks whether a text field is being edited, so single-letter app keys (q, ?, t, m) stay out of the way.
/// A field declares itself on every render with the App's frame number. The App's letter bindings ask
/// at key time whether a declaration was made in the latest frame: a field that is no longer rendered
/// (unmounted while editing) therefore stops counting as soon as the screen has been redrawn once,
/// although iocraft has no unmount hook. Nothing here is a State: declaring never redraws.
#[derive(Clone)]
pub struct TypingState {
    /// (frame, field) of the last editing declaration.
    seen: Arc<Mutex<Option<(u64, RegionId)>>>,
    /// The App's current frame, bumped at the start of every App render.
    frame: Arc<AtomicU64>,
}

impl TypingState {
    pub fn new() -> Self {
        Self { seen: Arc::new(Mutex::new(None)), frame: Arc::new(AtomicU64::new(0)) }
    }

    /// Called by the App at the start of every render.
    pub fn next_frame(&self) {
        self.frame.fetch_add(1, Ordering::Relaxed);
    }

    /// Whether a field declared itself editing in the latest drawn frame. Ask at key time, not at render time.
    pub fn typing(&self) -> bool {
        let frame = self.frame.load(Ordering::Relaxed);
        self.seen.lock().expect("typing").is_some_and(|(seen, _)| seen == frame)
    }

    fn declare(&self, id: RegionId, editing: bool) {
        let frame = self.frame.load(Ordering::Relaxed);
        let mut seen = self.seen.lock().expect("typing");
        if editing {
            *seen = Some((frame, id));
        } else if seen.is_some_and(|(_, field)| field == id) {
            *seen = None;
        }
    }
}

impl Default for TypingState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait UseTyping {
    /// Called by text fields every render: while `editing`, the app treats printable keys as text.
    fn use_typing(&mut self, editing: bool);
}

impl UseTyping for Hooks<'_, '_> {
    fn use_typing(&mut self, editing: bool) {
        let id = self.use_const(super::focus_next_field_id);
        if let Some(state) = self.try_use_context::<TypingState>() {
            state.declare(id, editing);
        }
    }
}
