use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

pub type RegionId = u64;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// A fresh id for a field or region.
pub(crate) fn focus_next_field_id() -> RegionId {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Copy, Debug)]
struct Region {
    id: RegionId,
    top: i32,
    left: i32,
    descend_on_enter: bool,
    frame: u64,
}

/// Keeps the focusable regions of an App and which one owns the keyboard. Regions re-register with
/// their rect on every render; the order is the order on screen (top to bottom, then left to right),
/// so a region that appears or disappears (a conditional sidebar) slots in where it is drawn.
#[derive(Default)]
pub struct FocusRegistry {
    regions: Vec<Region>,
    frame: u64,
}

impl FocusRegistry {
    pub fn next_frame(&mut self) {
        self.frame += 1;
    }

    pub fn register(&mut self, id: RegionId, top: i32, left: i32, descend_on_enter: bool) {
        let frame = self.frame;
        self.regions.retain(|r| r.id != id);
        self.regions.push(Region { id, top, left, descend_on_enter, frame });
    }

    fn live(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = self.regions.iter().filter(|r| r.frame + 1 >= self.frame).copied().collect();
        regions.sort_by_key(|r| (r.top, r.left));
        regions
    }

    /// The region `delta` steps away in screen order. Wrapping around is for tab; esc and enter stop at the ends.
    pub fn neighbour(&self, current: Option<RegionId>, delta: i32, wrap: bool) -> Option<RegionId> {
        let ordered = self.live();
        if ordered.is_empty() {
            return None;
        }
        let index = current.and_then(|id| ordered.iter().position(|r| r.id == id)).unwrap_or(0) as i32;
        let next = index + delta;
        let len = ordered.len() as i32;
        let picked = if wrap { (next % len + len) % len } else { next.clamp(0, len - 1) };
        Some(ordered[picked as usize].id)
    }

    pub fn first(&self) -> Option<RegionId> {
        self.live().first().map(|r| r.id)
    }

    pub fn descends(&self, id: Option<RegionId>) -> bool {
        id.and_then(|id| self.regions.iter().find(|r| r.id == id)).is_some_and(|r| r.descend_on_enter)
    }
}

/// The App's focus state: who is focused, and the registry.
#[derive(Clone)]
pub struct FocusState {
    pub registry: Arc<Mutex<FocusRegistry>>,
    pub focused: State<Option<RegionId>>,
}

impl FocusState {
    pub fn move_by(&self, delta: i32, wrap: bool) {
        let registry = self.registry.lock().expect("focus registry");
        let next = registry.neighbour(self.focused.get(), delta, wrap);
        drop(registry);
        let mut focused = self.focused;
        focused.set(next);
    }

    pub fn focus(&self, id: RegionId) {
        let mut focused = self.focused;
        focused.set(Some(id));
    }
}

pub trait UseFocusRegion {
    /// Registers this component as a focusable region. Returns (id, focused).
    fn use_focus_region(&mut self, descend_on_enter: bool) -> (RegionId, bool);
}

impl UseFocusRegion for Hooks<'_, '_> {
    fn use_focus_region(&mut self, descend_on_enter: bool) -> (RegionId, bool) {
        let id = self.use_const(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
        let rect = self.use_component_rect();
        let state = self.use_context::<FocusState>().clone();
        let (top, left) = rect.map(|r| (r.top, r.left)).unwrap_or((i32::MAX, i32::MAX));
        let mut registry = state.registry.lock().expect("focus registry");
        registry.register(id, top, left, descend_on_enter);
        // The first region to appear takes the focus.
        let focused = match state.focused.get() {
            None => {
                let first = registry.first();
                drop(registry);
                let mut focused = state.focused;
                focused.set(first);
                first == Some(id)
            }
            Some(current) => {
                drop(registry);
                current == id
            }
        };
        (id, focused)
    }
}
