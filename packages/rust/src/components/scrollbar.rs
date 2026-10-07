use iocraft::prelude::*;

use crate::theme::{Glyphs, Theme};

/// Where the thumb sits on a track of `rows` cells, for `visible` of `total` lines starting at `offset`.
pub struct ScrollThumb;

impl ScrollThumb {
    pub fn place(rows: usize, total: usize, visible: usize, offset: usize) -> (usize, usize) {
        if total <= visible || rows == 0 {
            return (0, rows);
        }
        let size = ((rows * visible) as f32 / total as f32).round().max(1.0) as usize;
        let start = (((rows - size) * offset) as f32 / (total - visible) as f32).round() as usize;
        (start, size)
    }
}

#[derive(Default, Props)]
pub struct ScrollbarProps {
    pub rows: u16,
    pub total: u32,
    pub visible: u32,
    pub offset: u32,
}

/// A one-column scrollbar: a faint track with a thumb whose size and position show how much is above and below. Hidden when everything fits.
#[component]
pub fn Scrollbar(props: &mut ScrollbarProps, hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let rows = props.rows as usize;
    if props.total <= props.visible {
        return element! { View(width: 1, height: props.rows) }.into_any();
    }
    let (start, size) = ScrollThumb::place(rows, props.total as usize, props.visible as usize, props.offset as usize);
    element! {
        View(width: 1, height: props.rows, flex_direction: FlexDirection::Column) {
            #((0..rows).map(|row| {
                let thumb = row >= start && row < start + size;
                element! { View(key: row, height: 1) { Text(content: if thumb { Glyphs::THUMB } else { Glyphs::TRACK }, color: if thumb { theme.tokens.text_muted } else { theme.tokens.surface_raised }) } }
            }))
        }
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_the_track_when_everything_fits() {
        assert_eq!(ScrollThumb::place(5, 3, 5, 0), (0, 5));
    }

    #[test]
    fn sizes_the_thumb_by_the_visible_share_and_moves_it_to_the_end() {
        assert_eq!(ScrollThumb::place(6, 12, 6, 0), (0, 3));
        assert_eq!(ScrollThumb::place(6, 12, 6, 6), (3, 3));
        assert_eq!(ScrollThumb::place(6, 60, 6, 27), (3, 1));
    }
}
