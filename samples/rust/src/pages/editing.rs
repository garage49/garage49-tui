use garage49_tui_iocraft::{Binding, Input, Label, LabelVariant, MainFocus, TextArea, TextBuffer, UseKeys, UseStatus};
use iocraft::prelude::*;

#[component]
pub fn EditingPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let field = hooks.use_state(|| 0usize);
    let notes = hooks.use_state(|| TextBuffer::from_text(&["Multi-line notes.", "한글도 됩니다.", "", "Arrows move, enter splits, backspace joins.", "PageUp/PageDown jump a page.", "The wheel scrolls the view;", "moving the cursor brings it back.", "", "line 9", "line 10", "line 11", "line 12 · the end"].join("\n")));
    let command = hooks.use_state(String::new);
    let current = field.get();
    let step = move |delta: i32| { let mut field = field; field.set((current as i32 + delta).clamp(0, 1) as usize); };
    hooks.use_keys(focused && current != 0, vec![Binding::new(&["up"], move || step(-1)), Binding::new(&["down"], move || step(1))], None);
    element! {
        View(flex_direction: FlexDirection::Column, width: 64, flex_shrink: 0.0_f32) {
            Label(content: "↑ on the first line / ↓ on the last line leaves the text area · click focuses", variant: LabelVariant::Muted)
            View(height: 1)
            TextArea(label: "Notes", buffer: notes.read().clone(), rows: 6u16, placeholder: "Write several lines…".to_string(), focused: focused && current == 0,
                on_change: move |b: TextBuffer| { let mut notes = notes; notes.set(b) }, on_focus: move |_| { let mut field = field; field.set(0) }, on_leave: move |d: i32| step(d))
            View(height: 1)
            Input(value: command.read().clone(), placeholder: "Type a command and press enter…".to_string(), focused: focused && current == 1,
                on_change: move |v: String| { let mut command = command; command.set(v) },
                on_submit: move |v: String| { status.report(&format!("submitted: {}", if v.is_empty() { "(empty)".to_string() } else { v })); let mut command = command; command.set(String::new()) },
                on_focus: move |_| { let mut field = field; field.set(1) }) {
                Label(content: "enter submits · the bar turns blue when focused", variant: LabelVariant::Muted)
            }
        }
    }
}
