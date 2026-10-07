use iocraft::prelude::*;

use super::scrollbar::Scrollbar;
use crate::input::{MouseLayer, UseMouse};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn label(self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogEntry {
    pub time: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Default, Props)]
pub struct LogViewProps {
    pub entries: Vec<LogEntry>,
    /// Rows scrolled up from the bottom; 0 follows the newest entry.
    pub offset: u32,
    pub on_scroll: HandlerMut<'static, u32>,
    /// Visible rows (the page gives it; iocraft has no late measurement for the first draw).
    pub rows: u16,
}

/// A scrolling log: fixed time column, level colored by severity, newest at the bottom. The wheel scrolls; a scrollbar shows the position.
#[component]
pub fn LogView(props: &mut LogViewProps, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme = hooks.use_context::<Theme>().clone();
    let t = theme.tokens;
    let height = props.rows.max(1) as usize;
    let total = props.entries.len();
    let max_offset = total.saturating_sub(height) as u32;
    let clamped = props.offset.min(max_offset);
    let end = total - clamped as usize;
    let visible: Vec<LogEntry> = props.entries[end.saturating_sub(height)..end].to_vec();
    let allowed = hooks.mouse_allowed(MouseLayer::Screen);
    {
        let mut on_scroll = props.on_scroll.take();
        hooks.use_mouse(allowed, move |event| match event.kind {
            MouseEventKind::ScrollUp => on_scroll((clamped + 3).min(max_offset)),
            MouseEventKind::ScrollDown => on_scroll(clamped.saturating_sub(3)),
            _ => {}
        });
    }
    let level_color = move |level: LogLevel| match level {
        LogLevel::Debug => t.text_muted,
        LogLevel::Info => t.accent_secondary,
        LogLevel::Warn => t.warning,
        LogLevel::Error => t.error,
    };
    element! {
        View(flex_direction: FlexDirection::Row, height: props.rows, overflow: Overflow::Hidden) {
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, overflow: Overflow::Hidden) {
                #(visible.iter().enumerate().map(|(index, entry)| element! {
                    View(key: index, flex_direction: FlexDirection::Row) {
                        View(width: 9, flex_shrink: 0.0_f32) { Text(content: entry.time.clone(), color: t.text_muted) }
                        View(width: 6, flex_shrink: 0.0_f32) { Text(content: entry.level.label(), weight: Weight::Bold, color: level_color(entry.level)) }
                        Text(content: entry.message.clone(), color: if entry.level == LogLevel::Error { t.error } else { t.text }, wrap: TextWrap::NoWrap)
                    }
                }))
            }
            Scrollbar(rows: props.rows, total: total as u32, visible: height as u32, offset: (end.saturating_sub(height)) as u32)
        }
    }
}
