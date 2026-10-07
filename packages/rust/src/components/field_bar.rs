use iocraft::prelude::*;

use super::focus_region::RegionSurface;
use crate::theme::{Glyphs, Theme};

#[derive(Default, Props)]
pub struct FieldBarProps {
    pub focused: bool,
    /// The color under the bar: the field's value surface unless the field has none and names its container's color.
    pub surface: Option<RegionSurface>,
    /// Height of the field; the bar covers every row.
    pub rows: Option<u16>,
}

/// The first column of a form field: blank normally, a blue ┃ down every row while the field is being edited.
/// Same rule as FocusRegion, at field scale: the bar is painted over the field's own first column.
#[component]
pub fn FieldBar(props: &mut FieldBarProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let rows = props.rows.unwrap_or(1).max(1);
    let surface = FieldBarSurface::resolve(props.surface);
    let glyph = if props.focused { Glyphs::BAR } else { " " };
    let content = (0..rows).map(|_| glyph).collect::<Vec<_>>().join("\n");
    element! {
        View(width: 1, height: rows, background_color: surface.color(&theme)) {
            Text(content: content, color: theme.tokens.accent_secondary)
        }
    }
}

/// The color a field bar is painted on: the value surface by default (TextField, TextArea, Select, Toggle),
/// or the container's color that a bar-only field (Checkbox, RadioGroup) passes explicitly.
pub struct FieldBarSurface;

impl FieldBarSurface {
    pub fn resolve(explicit: Option<RegionSurface>) -> RegionSurface {
        explicit.unwrap_or(RegionSurface::Surface)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_field_with_a_value_box_paints_its_bar_on_the_value_surface() {
        assert_eq!(FieldBarSurface::resolve(None), RegionSurface::Surface);
    }

    #[test]
    fn a_bar_only_field_paints_its_bar_on_the_container_color_it_names() {
        assert_eq!(FieldBarSurface::resolve(Some(RegionSurface::Panel)), RegionSurface::Panel);
    }
}
