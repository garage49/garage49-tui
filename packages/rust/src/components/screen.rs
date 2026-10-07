use std::sync::Arc;

use iocraft::prelude::*;

use crate::input::{MouseLayer, MouseState};
use crate::theme::Theme;

/// The overlay's rectangle (left, top, right, bottom) once drawn, for the backdrop's outside-click test.
pub type OverlayRect = State<Option<(i32, i32, i32, i32)>>;

/// Builds the overlay's element on every render; captures must be Send + Sync (states are).
pub type OverlayFactory = Arc<dyn Fn() -> AnyElement<'static> + Send + Sync>;

/// What is shown on top of the screen: an element factory and, optionally, where to anchor it (centered otherwise).
#[derive(Clone)]
pub struct OverlayRequest {
    pub factory: OverlayFactory,
    pub top: Option<i32>,
    pub left: Option<i32>,
}

/// The screen's overlay slot: show one element on top of everything; a click outside hides it.
#[derive(Clone, Copy)]
pub struct OverlayHandle {
    shown: State<Option<OverlayRequest>>,
}

impl OverlayHandle {
    pub fn show(&self, factory: OverlayFactory) {
        let mut shown = self.shown;
        shown.set(Some(OverlayRequest { factory, top: None, left: None }));
    }

    pub fn show_at(&self, factory: OverlayFactory, top: i32, left: i32) {
        let mut shown = self.shown;
        shown.set(Some(OverlayRequest { factory, top: Some(top), left: Some(left) }));
    }

    pub fn hide(&self) {
        let mut shown = self.shown;
        shown.set(None);
    }

    pub fn is_open(&self) -> bool {
        self.shown.read().is_some()
    }
}

#[derive(Default, Props)]
pub struct ThemeRootProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub theme: Option<Theme>,
}

/// Holds the app's theme as state so a settings screen can switch it; wraps the whole app once.
#[component]
pub fn ThemeRoot<'a>(props: &mut ThemeRootProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let initial = props.theme.clone().unwrap_or_else(Theme::opencode);
    let theme = hooks.use_state(move || initial);
    let current = theme.read().clone();
    element! {
        ContextProvider(value: Context::owned(ThemeSwitch { theme })) {
            ContextProvider(value: Context::owned(current)) {
                #(props.children.iter_mut())
            }
        }
    }
}

/// The setter installed by ThemeRoot.
#[derive(Clone, Copy)]
pub struct ThemeSwitch {
    pub theme: State<Theme>,
}

#[derive(Default, Props)]
pub struct ScreenProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub min_columns: Option<u16>,
    pub min_rows: Option<u16>,
}

/// The full-window root: fills the terminal with the background color, hosts the overlay slot and owns the mouse.
#[component]
pub fn Screen<'a>(props: &mut ScreenProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let (columns, rows) = hooks.use_terminal_size();
    let theme = hooks.use_context::<Theme>().clone();
    let shown = hooks.use_state(|| None::<OverlayRequest>);
    let enabled = hooks.use_state(|| true);
    let overlay_open = hooks.use_state(|| false);
    let open = shown.read().is_some();
    if overlay_open.get() != open {
        let mut overlay_open_state = overlay_open;
        overlay_open_state.set(open);
    }
    let mut system = hooks.use_context_mut::<SystemContext>();
    system.set_mouse_capture(enabled.get());
    drop(system);

    let min_columns = props.min_columns.unwrap_or(80);
    let min_rows = props.min_rows.unwrap_or(24);
    let handle = OverlayHandle { shown };
    let mouse = MouseState { enabled, overlay_open };

    // A click outside the overlay hides it: the backdrop is the whole screen on the overlay layer,
    // and the overlay's own components are drawn later, so they also receive the click (bubbling).
    let hide = handle;
    let overlay_rect: OverlayRect = hooks.use_state(|| None::<(i32, i32, i32, i32)>);
    hooks.use_terminal_events(move |event| {
        if !open {
            return;
        }
        if let TerminalEvent::FullscreenMouse(FullscreenMouseEvent { kind: MouseEventKind::Down(_), column, row, .. }) = event {
            let inside = overlay_rect.get().is_some_and(|(l, t, r, b)| (column as i32) >= l && (column as i32) < r && (row as i32) >= t && (row as i32) < b);
            if !inside {
                hide.hide();
            }
        }
    });

    if columns < min_columns || rows < min_rows {
        return element! {
            View(width: columns, height: rows, background_color: theme.tokens.background, align_items: AlignItems::Center, justify_content: JustifyContent::Center) {
                Text(content: format!("Terminal too small: {columns}×{rows}, need {min_columns}×{min_rows}"), color: theme.tokens.text_muted)
            }
        }
        .into_any();
    }
    let below = if open { theme.dimmed(0.4) } else { theme.clone() };
    let request = shown.read().clone();
    element! {
        ContextProvider(value: Context::owned(handle)) {
            ContextProvider(value: Context::owned(mouse)) {
                View(width: columns, height: rows, background_color: below.tokens.background, flex_direction: FlexDirection::Column) {
                    ContextProvider(value: Context::owned(below.clone())) {
                        View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
                            #(props.children.iter_mut())
                        }
                    }
                    #(request.map(|request| element! {
                        ContextProvider(value: Context::owned(theme.clone())) {
                            ContextProvider(value: Context::owned(MouseLayer::Overlay)) {
                                OverlayLayer(request: request, columns: columns, rows: rows, rect: overlay_rect)
                            }
                        }
                    }))
                }
            }
        }
    }
    .into_any()
}

#[derive(Default, Props)]
struct OverlayLayerProps {
    request: Option<OverlayRequest>,
    columns: u16,
    rows: u16,
    rect: Option<OverlayRect>,
}

/// The overlay element, centered or anchored, on its own mouse layer; records its rect for the backdrop test.
#[component]
fn OverlayLayer(props: &mut OverlayLayerProps) -> impl Into<AnyElement<'static>> {
    let request = props.request.clone().expect("overlay request");
    let (columns, rows) = (props.columns, props.rows);
    let rect = props.rect;
    match (request.top, request.left) {
        (Some(top), Some(left)) => element! {
            View(position: Position::Absolute, top: top, left: left) {
                OverlayBox(request: request, rect: rect)
            }
        }
        .into_any(),
        _ => element! {
            View(position: Position::Absolute, top: 0, left: 0, width: columns, height: rows, align_items: AlignItems::Center, justify_content: JustifyContent::Center) {
                OverlayBox(request: request, rect: rect)
            }
        }
        .into_any(),
    }
}

#[derive(Default, Props)]
struct OverlayBoxProps {
    request: Option<OverlayRequest>,
    rect: Option<OverlayRect>,
}

#[component]
fn OverlayBox(props: &mut OverlayBoxProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let measured = hooks.use_component_rect();
    if let (Some(mut state), Some(r)) = (props.rect, measured) {
        let next = Some((r.left, r.top, r.right, r.bottom));
        if state.get() != next {
            state.set(next);
        }
    }
    let request = props.request.clone().expect("overlay request");
    let element = (request.factory)();
    element! { View { #(std::iter::once(element)) } }
}
