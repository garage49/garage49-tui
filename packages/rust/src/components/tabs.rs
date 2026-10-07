use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};
use crate::theme::{Glyphs, TextWidth, Theme};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

impl Tab {
    pub fn new(id: &str, label: &str) -> Self {
        Self { id: id.into(), label: label.into() }
    }
}

/// large: two rows, a thin ▔ under the active label (top navigation). small: one row, the active label filled (inside pages).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TabsSize {
    #[default]
    Small,
    Large,
}

#[derive(Default, Props)]
pub struct TabsProps {
    pub tabs: Vec<Tab>,
    pub active_id: String,
    pub focused: bool,
    pub size: TabsSize,
    /// large: the background of the label row (the header's surface).
    pub label_surface: Option<Color>,
    pub on_change: HandlerMut<'static, Tab>,
}

/// Tabs in two sizes. The active tab is the cursor: accent while the tabs are focused, muted/raised otherwise. Click selects.
#[component]
pub fn Tabs(props: &mut TabsProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let widths: Vec<u16> = props.tabs.iter().map(|tab| TextWidth::of(&tab.label) as u16 + 4).collect();
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let tabs = props.tabs.clone();
        let widths = widths.clone();
        let mut on_change = props.on_change.take();
        hooks.use_mouse(allowed, move |event| {
            if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                return;
            }
            let mut x = event.local_x;
            for (index, width) in widths.iter().enumerate() {
                if x < *width as i32 {
                    if let Some(tab) = tabs.get(index) {
                        on_change(tab.clone());
                    }
                    return;
                }
                x -= *width as i32;
            }
        });
    }
    let focused = props.focused;
    let active_id = props.active_id.clone();
    let label_surface = props.label_surface;
    let size = props.size;
    let t = theme.tokens;
    element! {
        View(flex_direction: FlexDirection::Row, height: if size == TabsSize::Small { 1 } else { 2 }) {
            #(props.tabs.iter().zip(widths.iter()).enumerate().map(|(index, (tab, width))| {
                let active = tab.id == active_id;
                let paint = TabPaint { active, focused, label_surface, tokens: t };
                match size {
                    TabsSize::Small => small_tab(index, tab, &paint),
                    TabsSize::Large => large_tab(index, tab, *width, &paint),
                }
            }))
        }
    }
}

/// How one tab is drawn: its state and the theme's tokens.
struct TabPaint {
    active: bool,
    focused: bool,
    label_surface: Option<Color>,
    tokens: crate::theme::ThemeTokens,
}

/// One small tab: the active label filled with the cursor color.
fn small_tab(index: usize, tab: &Tab, paint: &TabPaint) -> AnyElement<'static> {
    let (active, focused, t) = (paint.active, paint.focused, &paint.tokens);
    let highlighted = active && focused;
    let fill = if active { Some(if focused { t.selection_background } else { t.surface_raised }) } else { None };
    let color = if highlighted { t.selection_text } else if active { t.text_bright } else { t.text_muted };
    element! {
        View(key: index, padding_left: 2, padding_right: 2, background_color: fill) {
            Text(content: tab.label.clone(), weight: if active { Weight::Bold } else { Weight::Normal }, color: color)
        }
    }
    .into_any()
}

/// One large tab: the label on the header surface, a thin ▔ below it when active.
fn large_tab(index: usize, tab: &Tab, width: u16, paint: &TabPaint) -> AnyElement<'static> {
    let (active, focused, label_surface, t) = (paint.active, paint.focused, paint.label_surface, &paint.tokens);
    let line = if active { Glyphs::OVERLINE.repeat(width as usize - 2) } else { String::new() };
    element! {
        View(key: index, flex_direction: FlexDirection::Column, width: width) {
            View(height: 1, padding_left: 2, padding_right: 2, background_color: label_surface) {
                Text(content: tab.label.clone(), weight: if active { Weight::Bold } else { Weight::Normal }, color: if active { t.text_bright } else { t.text_muted })
            }
            View(height: 1, padding_left: 1, padding_right: 1) {
                Text(content: line, color: if focused { t.accent } else { t.text_muted })
            }
        }
    }
    .into_any()
}
