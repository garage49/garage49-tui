use iocraft::prelude::*;

use crate::input::{MouseLayer, UseMouse};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Column {
    pub title: String,
    pub width: u16,
    pub align: Align,
}

impl Column {
    pub fn new(title: &str, width: u16) -> Self {
        Self { title: title.into(), width, align: Align::Left }
    }
    pub fn right(mut self) -> Self {
        self.align = Align::Right;
        self
    }
}

/// One table row: an id and the cell texts in column order.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Row {
    pub id: String,
    pub cells: Vec<String>,
}

impl Row {
    pub fn new(id: &str, cells: &[&str]) -> Self {
        Self { id: id.into(), cells: cells.iter().map(|c| c.to_string()).collect() }
    }
}

#[derive(Default, Props)]
pub struct TableProps {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    pub selected_id: Option<String>,
    pub focused: bool,
    pub on_click: HandlerMut<'static, Row>,
    pub on_select: HandlerMut<'static, Row>,
}

/// Fixed-width columns with a muted bold header; the selected row is filled with the accent color. Click selects.
#[component]
pub fn Table(props: &mut TableProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let rows = props.rows.clone();
        let selected = props.selected_id.clone();
        let mut on_click = props.on_click.take();
        let mut on_select = props.on_select.take();
        hooks.use_mouse(allowed, move |event| match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if event.local_y >= 1 {
                    if let Some(row) = rows.get(event.local_y as usize - 1) {
                        on_click(row.clone());
                    }
                }
            }
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                let delta = if event.kind == MouseEventKind::ScrollUp { -1 } else { 1 };
                let index = rows.iter().position(|r| Some(&r.id) == selected.as_ref()).unwrap_or(0) as i32;
                if let Some(row) = rows.get((index + delta).clamp(0, rows.len() as i32 - 1) as usize) {
                    on_select(row.clone());
                }
            }
            _ => {}
        });
    }
    let columns = props.columns.clone();
    let line = move |cells: Vec<String>, color: Color, bold: bool| -> AnyElement<'static> {
        let columns = columns.clone();
        element! {
            View(flex_direction: FlexDirection::Row) {
                #(columns.iter().enumerate().map(|(index, column)| {
                    let text = cells.get(index).cloned().unwrap_or_default();
                    element! {
                        View(key: index, width: column.width, flex_shrink: 0.0_f32, margin_right: 2, justify_content: if column.align == Align::Right { JustifyContent::End } else { JustifyContent::Start }) {
                            Text(content: text, weight: if bold { Weight::Bold } else { Weight::Normal }, color: color, wrap: TextWrap::NoWrap)
                        }
                    }
                }))
            }
        }
        .into_any()
    };
    let header = line(props.columns.iter().map(|c| c.title.clone()).collect(), t.text_muted, true);
    let focused = props.focused;
    let selected = props.selected_id.clone();
    element! {
        View(flex_direction: FlexDirection::Column) {
            View(padding_left: 1, padding_right: 1) { #(std::iter::once(header)) }
            #(props.rows.iter().enumerate().map(|(index, row)| {
                let is_selected = selected.as_deref() == Some(row.id.as_str());
                let highlighted = is_selected && focused;
                let fill = if is_selected { Some(if focused { t.selection_background } else { t.surface_raised }) } else { None };
                let body = line(row.cells.clone(), if highlighted { t.selection_text } else { t.text }, is_selected);
                element! { View(key: index, padding_left: 1, padding_right: 1, background_color: fill) { #(std::iter::once(body)) } }
            }))
        }
    }
}
