use iocraft::prelude::*;

use super::focus_region::RegionSurface;
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct FieldBarProps {
    pub focused: bool,
    pub surface: RegionSurface,
    /// Height of the field; the bar covers every row.
    pub rows: Option<u16>,
}

/// The first column of a form field: blank normally, a blue ┃ down every row while the field is being edited.
/// Same rule as FocusRegion, at field scale: the bar is painted over the field's own first column.
#[component]
pub fn FieldBar(props: &mut FieldBarProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let rows = props.rows.unwrap_or(1).max(1);
    let surface = if props.surface == RegionSurface::Background { RegionSurface::Surface } else { props.surface };
    let glyph = if props.focused { Glyphs::BAR } else { " " };
    let content = (0..rows).map(|_| glyph).collect::<Vec<_>>().join("\n");
    element! {
        View(width: 1, height: rows, background_color: surface.color(&theme)) {
            Text(content: content, color: theme.tokens.accent_secondary)
        }
    }
}
