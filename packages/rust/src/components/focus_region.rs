use iocraft::prelude::*;

use crate::theme::{Glyphs, Theme};

/// What a region draws in its first column; the focus bar is painted on that color.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RegionSurface {
    #[default]
    Background,
    Panel,
    Surface,
    Heading,
}

impl RegionSurface {
    pub fn color(self, theme: &Theme) -> Color {
        match self {
            RegionSurface::Background => theme.tokens.background,
            RegionSurface::Panel => theme.tokens.panel,
            RegionSurface::Surface => theme.tokens.surface,
            RegionSurface::Heading => theme.tokens.heading,
        }
    }
}

#[derive(Default, Props)]
pub struct FocusRegionProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    pub focused: bool,
    /// Take the remaining width of the parent row.
    pub grow: bool,
    pub surface: RegionSurface,
    /// How many rows the bar covers from the top; the whole region by default. A header marks only its title row.
    pub rows: Option<u16>,
}

/// A focusable region. When focused, a column of ┃ in accentSecondary (blue) is painted over the
/// region's first column; otherwise nothing is drawn. The region keeps its first column free (padding-left 1).
#[component]
pub fn FocusRegion<'a>(props: &mut FocusRegionProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let rect = hooks.use_component_rect();
    let height = rect.map(|r| (r.bottom - r.top).max(1) as u16).unwrap_or(1);
    let bar_height = props.rows.map(|rows| rows.min(height)).unwrap_or(height);
    let bar = Glyphs::BAR.to_string();
    let surface = props.surface.color(&theme);
    let focused = props.focused;
    let (grow, shrink) = if props.grow { (1.0_f32, 1.0_f32) } else { (0.0_f32, 0.0_f32) };
    element! {
        View(flex_direction: FlexDirection::Row, flex_grow: grow, flex_shrink: shrink) {
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
                #(props.children.iter_mut())
            }
            #(if focused { Some(element! {
                View(position: Position::Absolute, top: 0, left: 0, width: 1, height: bar_height, flex_direction: FlexDirection::Column, background_color: surface) {
                    #((0..bar_height).map(|row| element! {
                        View(key: row, height: 1) { Text(content: bar.clone(), color: theme.tokens.accent_secondary) }
                    }))
                }
            }) } else { None })
        }
    }
}
