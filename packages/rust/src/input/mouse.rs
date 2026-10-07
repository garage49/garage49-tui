use iocraft::prelude::*;

/// Shared mouse state: whether reporting is on, and whether an overlay owns the pointer.
/// iocraft delivers "local" mouse events to every component under the pointer, so layering is a
/// flag: while an overlay is open, only components inside it (MouseLayer::Overlay) react.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MouseLayer {
    #[default]
    Screen,
    Overlay,
}

#[derive(Clone, Copy)]
pub struct MouseState {
    pub enabled: State<bool>,
    pub overlay_open: State<bool>,
}

/// A local mouse event with the component-relative position.
pub struct LocalMouse {
    pub kind: MouseEventKind,
    pub local_x: i32,
    pub local_y: i32,
}

pub trait UseMouse {
    /// Whether this component may react to the mouse: reporting is on and it is on the top layer.
    fn mouse_allowed(&self, layer: MouseLayer) -> bool;
    /// Calls `on_event` with local coordinates for mouse presses and wheel ticks inside this component.
    fn use_mouse(&mut self, allowed: bool, on_event: impl FnMut(LocalMouse) + Send + 'static);
}

impl UseMouse for Hooks<'_, '_> {
    fn mouse_allowed(&self, layer: MouseLayer) -> bool {
        let state = self.try_use_context::<MouseState>().map(|s| *s);
        let in_overlay = self.try_use_context::<MouseLayer>().map(|l| *l == MouseLayer::Overlay).unwrap_or(false);
        match state {
            None => false,
            Some(state) => state.enabled.get() && (!state.overlay_open.get() || in_overlay || layer == MouseLayer::Overlay),
        }
    }

    fn use_mouse(&mut self, allowed: bool, mut on_event: impl FnMut(LocalMouse) + Send + 'static) {
        // iocraft already filters local events to this component and makes row/column relative to it.
        self.use_local_terminal_events(move |event| {
            if !allowed {
                return;
            }
            let TerminalEvent::FullscreenMouse(mouse) = event else { return };
            let kind = mouse.kind;
            match kind {
                MouseEventKind::Down(_) | MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {}
                _ => return,
            }
            on_event(LocalMouse { kind, local_x: mouse.column as i32, local_y: mouse.row as i32 });
        });
    }
}
