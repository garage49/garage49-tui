use iocraft::prelude::*;

/// The status line's "last action" slot: report("saved") shows it at the bottom left.
#[derive(Clone, Copy)]
pub struct StatusState {
    pub last_action: State<String>,
}

impl StatusState {
    pub fn report(&self, text: &str) {
        let mut last = self.last_action;
        last.set(text.to_string());
    }
}

pub trait UseStatus {
    fn use_status(&self) -> StatusState;
}

impl UseStatus for Hooks<'_, '_> {
    fn use_status(&self) -> StatusState {
        *self.use_context::<StatusState>()
    }
}
