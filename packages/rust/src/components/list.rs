use iocraft::prelude::*;

use super::list_layout::{ListLayout, ListRow};
use super::truncate::Truncate;
use crate::input::{MouseLayer, UseMouse};
use crate::theme::Theme;

/// A row of a List: a label, and optionally a shortcut (muted) or a value (text color) on the right.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ListItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub value: Option<String>,
    pub section: Option<String>,
}

impl ListItem {
    pub fn new(id: &str, label: &str) -> Self {
        Self { id: id.into(), label: label.into(), ..Default::default() }
    }
    pub fn shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn section(mut self, section: &str) -> Self {
        self.section = Some(section.into());
        self
    }
}

#[derive(Default, Props)]
pub struct ListProps {
    pub items: Vec<ListItem>,
    pub selected_id: Option<String>,
    /// Whether the list owns keyboard focus; an unfocused list shows its selection on a raised surface.
    pub focused: bool,
    pub width: Option<u16>,
    /// A left click on a row.
    pub on_click: HandlerMut<'static, ListItem>,
    /// The wheel moving the selection to a neighbouring row.
    pub on_select: HandlerMut<'static, ListItem>,
}

/// Rows grouped under bold colored section headings; the selected row is filled with the accent color.
#[component]
pub fn List(props: &mut ListProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let rows = ListLayout::rows(&props.items);
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let rows = rows.clone();
        let items = props.items.clone();
        let selected = props.selected_id.clone();
        let mut on_click = props.on_click.take();
        let mut on_select = props.on_select.take();
        hooks.use_mouse(allowed, move |event| match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(item) = ListLayout::item_at(&rows, event.local_y) {
                    on_click(item.clone());
                }
            }
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                let delta = if event.kind == MouseEventKind::ScrollUp { -1 } else { 1 };
                let index = items.iter().position(|item| Some(&item.id) == selected.as_ref()).unwrap_or(0) as i32;
                let next = (index + delta).clamp(0, items.len() as i32 - 1);
                if let Some(item) = items.get(next as usize) {
                    on_select(item.clone());
                }
            }
            _ => {}
        });
    }
    let focused = props.focused;
    let selected = props.selected_id.clone();
    let width = props.width;
    let body = element! {
        View(flex_direction: FlexDirection::Column) {
            #(rows.into_iter().enumerate().map(|(index, row)| match row {
                ListRow::Gap => element! { View(key: index, height: 1) }.into_any(),
                ListRow::Section(title) => element! {
                    View(key: index) { Text(content: title, weight: Weight::Bold, color: theme.tokens.heading) }
                }.into_any(),
                ListRow::Item(item) => {
                    let is_selected = selected.as_deref() == Some(item.id.as_str());
                    let highlighted = is_selected && focused;
                    let fill = if is_selected { Some(if focused { theme.tokens.selection_background } else { theme.tokens.surface_raised }) } else { None };
                    let color = if highlighted { theme.tokens.selection_text } else { theme.tokens.text };
                    let trailing = item.value.clone().or(item.shortcut.clone());
                    let trailing_color = if highlighted { theme.tokens.selection_text } else if item.value.is_some() { theme.tokens.text } else { theme.tokens.text_muted };
                    let weight = if is_selected { Weight::Bold } else { Weight::Normal };
                    element! {
                        View(key: index, background_color: fill, flex_direction: FlexDirection::Row) {
                            Truncate(content: item.label, weight: weight, color: color)
                            #(trailing.map(|text| element! { View(flex_shrink: 0.0_f32, padding_left: 2) { Text(content: text, color: trailing_color) } }))
                        }
                    }.into_any()
                }
            }))
        }
    };
    match width {
        Some(width) => element! { View(width: width) { #(std::iter::once(body.into_any())) } }.into_any(),
        None => body.into_any(),
    }
}
