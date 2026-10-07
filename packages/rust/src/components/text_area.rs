use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::field_bar::FieldBar;
use super::scrollbar::Scrollbar;
use super::text_buffer::TextBuffer;
use crate::input::{Binding, MouseLayer, UseKeys, UseMouse};
use crate::shell::UseTyping;
use crate::theme::Theme;

#[derive(Default, Props)]
pub struct TextAreaProps {
    pub label: String,
    pub buffer: Option<TextBuffer>,
    pub on_change: HandlerMut<'static, TextBuffer>,
    pub placeholder: Option<String>,
    pub focused: bool,
    pub on_focus: HandlerMut<'static, ()>,
    pub label_width: Option<u16>,
    /// Visible rows; the view scrolls to keep the cursor inside.
    pub rows: Option<u16>,
    /// ↑ on the first line or ↓ on the last line: hand the focus to the neighbouring field (-1 / 1).
    pub on_leave: HandlerMut<'static, i32>,
}

/// A multi-line editor on a surface: arrows move the cursor, enter splits the line, backspace joins.
/// The cursor is the inverted cell. The wheel scrolls the view; moving the cursor brings it back into view.
#[component]
pub fn TextArea(props: &mut TextAreaProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    hooks.use_typing(props.focused);
    let buffer = props.buffer.clone().unwrap_or_else(|| TextBuffer::from_text(""));
    let rows = props.rows.unwrap_or(5).max(1) as usize;
    let scroll_top = hooks.use_state(|| 0usize);
    let followed = hooks.use_state(|| (0usize, 0usize));
    let max_top = buffer.lines.len().saturating_sub(rows);
    let mut top = scroll_top.get().min(max_top);
    // Follow the cursor whenever it moves (the buffer changed); the wheel moves the view alone.
    if followed.get() != (buffer.row, buffer.col) {
        if buffer.row < top {
            top = buffer.row;
        } else if buffer.row >= top + rows {
            top = buffer.row + 1 - rows;
        }
        let mut scroll_top = scroll_top;
        scroll_top.set(top);
        let mut followed = followed;
        followed.set((buffer.row, buffer.col));
    }
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let mut on_focus = props.on_focus.take();
        let mut scroll_top = scroll_top;
        hooks.use_mouse(allowed, move |event| match event.kind {
            MouseEventKind::Down(_) => on_focus(()),
            MouseEventKind::ScrollUp => scroll_top.set(top.saturating_sub(3)),
            MouseEventKind::ScrollDown => scroll_top.set((top + 3).min(max_top)),
            _ => {}
        });
    }
    let on_change = Arc::new(Mutex::new(props.on_change.take()));
    let on_leave = Arc::new(Mutex::new(props.on_leave.take()));
    let change = |f: Box<dyn Fn(&TextBuffer) -> TextBuffer + Send + Sync>| {
        let on_change = on_change.clone();
        let buffer = buffer.clone();
        move || (on_change.lock().expect("handler"))(f(&buffer))
    };
    let up = {
        let on_leave = on_leave.clone();
        let on_change = on_change.clone();
        let buffer = buffer.clone();
        move || if buffer.row == 0 { (on_leave.lock().expect("handler"))(-1) } else { (on_change.lock().expect("handler"))(buffer.move_by(-1, 0)) }
    };
    let down = {
        let on_leave = on_leave.clone();
        let on_change = on_change.clone();
        let buffer = buffer.clone();
        move || if buffer.row + 1 == buffer.lines.len() { (on_leave.lock().expect("handler"))(1) } else { (on_change.lock().expect("handler"))(buffer.move_by(1, 0)) }
    };
    let rows_i = rows as i32;
    let typed = {
        let on_change = on_change.clone();
        let buffer = buffer.clone();
        move |text: String| (on_change.lock().expect("handler"))(buffer.insert(&text))
    };
    hooks.use_keys(
        props.focused,
        vec![
            Binding::new(&["up"], up),
            Binding::new(&["down"], down),
            Binding::new(&["left"], change(Box::new(|b| b.move_by(0, -1)))),
            Binding::new(&["right"], change(Box::new(|b| b.move_by(0, 1)))),
            Binding::new(&["pageup"], change(Box::new(move |b| b.move_by(-rows_i, 0)))),
            Binding::new(&["pagedown"], change(Box::new(move |b| b.move_by(rows_i, 0)))),
            Binding::new(&["home"], change(Box::new(|b| b.home()))),
            Binding::new(&["end"], change(Box::new(|b| b.end()))),
            Binding::new(&["enter"], change(Box::new(|b| b.newline()))),
            Binding::new(&["backspace", "delete"], change(Box::new(|b| b.backspace()))),
        ],
        Some(Box::new(typed)),
    );
    let focused = props.focused;
    let empty = buffer.text().is_empty() && !focused;
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let lines: Vec<AnyElement<'static>> = (0..rows)
        .map(|index| {
            let row = top + index;
            let line = buffer.lines.get(row).cloned();
            let cursor = if focused && row == buffer.row { Some(buffer.col) } else { None };
            match (line, cursor) {
                (None, _) => element! { View(key: index, height: 1) { Text(content: " ") } }.into_any(),
                (Some(line), None) => element! { View(key: index, height: 1) { Text(content: if line.is_empty() { " ".to_string() } else { line }, color: t.text, wrap: TextWrap::NoWrap) } }.into_any(),
                (Some(line), Some(col)) => {
                    let chars: Vec<char> = line.chars().collect();
                    let before: String = chars[..col.min(chars.len())].iter().collect();
                    let under: String = chars.get(col).map(|c| c.to_string()).unwrap_or_else(|| " ".into());
                    let after: String = chars.iter().skip(col + 1).collect();
                    element! {
                        View(key: index, height: 1) {
                            MixedText(contents: vec![MixedTextContent::new(before).color(t.text), MixedTextContent::new(under).color(t.text).invert(), MixedTextContent::new(after).color(t.text)], wrap: TextWrap::NoWrap)
                        }
                    }.into_any()
                }
            }
        })
        .collect();
    element! {
        View(flex_direction: FlexDirection::Row, height: rows as u16) {
            View(width: props.label_width.unwrap_or(14), flex_shrink: 0.0_f32) { Text(content: props.label.clone(), color: if focused { t.text } else { t.text_muted }) }
            FieldBar(focused: focused, rows: rows as u16)
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, background_color: t.surface, padding_left: 1, padding_right: 1) {
                #(if empty { vec![element! { Text(content: placeholder.clone(), color: t.text_muted) }.into_any()] } else { lines })
            }
            View(background_color: t.surface) { Scrollbar(rows: rows as u16, total: buffer.lines.len() as u32, visible: rows as u32, offset: top as u32) }
        }
    }
}
