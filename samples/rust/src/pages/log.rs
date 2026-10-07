use std::time::Duration;

use garage49_tui_iocraft::{Binding, Label, LabelVariant, LogEntry, LogLevel, LogView, MainFocus, UseKeys, UseStatus, Section};
use iocraft::prelude::*;

const MESSAGES: [(LogLevel, &str); 7] = [
    (LogLevel::Info, "Server listening on :8080"),
    (LogLevel::Debug, "GET /api/projects 200 12ms"),
    (LogLevel::Info, "한글 로그 메시지도 폭이 맞습니다"),
    (LogLevel::Warn, "Slow query: SELECT * FROM sessions (812ms)"),
    (LogLevel::Debug, "Cache hit ratio 0.93"),
    (LogLevel::Error, "Upstream timeout after 30s: projects-sync"),
    (LogLevel::Info, "Reconnected to upstream"),
];

fn entry(count: usize) -> LogEntry {
    let (level, message) = MESSAGES[count % MESSAGES.len()];
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) % 86400;
    LogEntry { time: format!("{:02}:{:02}:{:02}", secs / 3600, (secs / 60) % 60, secs % 60), level, message: format!("{message} #{}", count + 1) }
}

#[component]
pub fn LogPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let entries = hooks.use_state(|| (0..30).map(entry).collect::<Vec<_>>());
    let offset = hooks.use_state(|| 0u32);
    let paused = hooks.use_state(|| false);
    hooks.use_future(async move {
        let mut entries = entries;
        loop {
            smol::Timer::after(Duration::from_millis(700)).await;
            if !paused.get() {
                let mut list = entries.read().clone();
                let count = list.len();
                list.push(entry(count));
                if list.len() > 500 { list.remove(0); }
                entries.set(list);
            }
        }
    });
    let current = offset.get();
    let is_paused = paused.get();
    let set = |value: u32| { move || { let mut offset = offset; offset.set(value); } };
    let toggle = move || { let mut paused = paused; paused.set(!is_paused); status.report(if is_paused { "log: resumed" } else { "log: paused" }); };
    hooks.use_keys(focused, vec![
        Binding::new(&["up", "k"], set(current + 1)), Binding::new(&["down", "j"], set(current.saturating_sub(1))),
        Binding::new(&["pageup"], set(current + 10)), Binding::new(&["pagedown"], set(current.saturating_sub(10))),
        Binding::new(&["end", "G"], set(0)), Binding::new(&["p"], toggle),
    ], None);
    let count = entries.read().len();
    let scroll = move |value: u32| { let mut offset = offset; offset.set(value); };
    element! {
        Section(title: "Server log".to_string(), grow: true) {
        View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32) {
            View(flex_direction: FlexDirection::Row) {
                Label(content: format!("{count} entries · "), variant: LabelVariant::Muted)
                Label(content: if is_paused { "paused" } else { "live" }, variant: if is_paused { LabelVariant::Warning } else { LabelVariant::Success })
                Label(content: " · ↑↓ scroll · G follow · p pause", variant: LabelVariant::Muted)
                #(if current > 0 { Some(element! { Label(content: format!("  ▲ {current} rows above the end"), variant: LabelVariant::Accent) }) } else { None })
            }
            View(height: 1)
            LogView(entries: entries.read().clone(), offset: current, on_scroll: scroll, rows: 20u16)
        }
        }
    }
}
