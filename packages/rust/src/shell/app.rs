use std::sync::{Arc, Mutex};

use iocraft::prelude::*;

use super::focus::{FocusRegistry, FocusState};
use super::status::StatusState;
use super::typing::TypingState;
use crate::components::{ConfirmDialog, HelpEntry, HelpOverlay, KeyHint, KeyHintBar, ListItem, OverlayHandle, Palette, Screen, StatusLine, StatusSegment, StatusTone, ThemeSwitch};
use crate::input::{Binding, MouseState, UseKeys};
use crate::theme::{Glyphs, Theme};

/// A palette entry with the action it runs.
#[derive(Clone)]
pub struct Command {
    pub item: ListItem,
    pub run: Arc<dyn Fn() + Send + Sync>,
}

impl Command {
    pub fn new(item: ListItem, run: impl Fn() + Send + Sync + 'static) -> Self {
        Self { item, run: Arc::new(run) }
    }
}

#[derive(Default, Props)]
pub struct AppProps<'a> {
    pub children: Vec<AnyElement<'a>>,
    /// Extra palette entries, before the built-in ones.
    pub commands: Vec<Command>,
    /// Extra help lines, after the built-in keys.
    pub help: Vec<HelpEntry>,
    /// Extra key hints at the bottom right, before the built-in ones.
    pub hints: Vec<KeyHint>,
    /// Extra status segments after the last action.
    pub status: Vec<StatusSegment>,
    /// Text at the bottom left of the key hint bar.
    pub context: String,
    /// The quit confirmation's message.
    pub quit_message: Option<String>,
    /// Show a notice instead of the app below this size; off by default (the App degrades instead).
    pub min_columns: Option<u16>,
    pub min_rows: Option<u16>,
}

/// The application frame: the screen, theme, mouse, overlay slot, focus cycling and the two bottom
/// bars. Compose `App > (Nav) + Content > (Sidebar) + Main`: Nav and Sidebar are optional, but at
/// least one of them must be present; everything else is a default.
#[component]
pub fn App<'a>(props: &mut AppProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let registry = hooks.use_const(|| Arc::new(Mutex::new(FocusRegistry::default()))).clone();
    let focused = hooks.use_state(|| None::<u64>);
    let last_action = hooks.use_state(|| "ready".to_string());
    let typing = hooks.use_const(TypingState::new).clone();
    registry.lock().expect("focus registry").next_frame();
    typing.next_frame();
    let focus = FocusState { registry, focused };
    let status = StatusState { last_action };
    element! {
        Screen(min_columns: props.min_columns, min_rows: props.min_rows) {
            ContextProvider(value: Context::owned(focus.clone())) {
                ContextProvider(value: Context::owned(status)) {
                    ContextProvider(value: Context::owned(typing)) {
                        Frame(commands: props.commands.clone(), help: props.help.clone(), hints: props.hints.clone(), status: props.status.clone(), context: props.context.clone(), quit_message: props.quit_message.clone()) {
                            #(props.children.iter_mut())
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default, Props)]
struct FrameProps<'a> {
    children: Vec<AnyElement<'a>>,
    commands: Vec<Command>,
    help: Vec<HelpEntry>,
    hints: Vec<KeyHint>,
    status: Vec<StatusSegment>,
    context: String,
    quit_message: Option<String>,
}

fn builtin_help() -> Vec<HelpEntry> {
    vec![
        HelpEntry::new("tab / shift+tab", "cycle focus through the regions, top to bottom"),
        HelpEntry::new("enter / esc", "go down into the next region / back up (arrows stay inside a region)"),
        HelpEntry::new("↑↓ / jk", "move in the focused list"),
        HelpEntry::new("ctrl+p", "command palette"),
        HelpEntry::new("t", "toggle theme"),
        HelpEntry::new("m", "toggle the mouse (off: the terminal selects text again; on: use shift+drag)"),
        HelpEntry::new("?  q", "help · quit"),
        HelpEntry::new("mouse", "click a region to focus it; click tabs, rows, buttons, hints; wheel scrolls"),
    ]
}

/// The App's built-in actions, each showing an overlay or toggling a setting.
#[derive(Clone)]
struct Actions {
    palette: Arc<dyn Fn() + Send + Sync>,
    help: Arc<dyn Fn() + Send + Sync>,
    quit: Arc<dyn Fn() + Send + Sync>,
    theme: Arc<dyn Fn() + Send + Sync>,
    mouse: Arc<dyn Fn() + Send + Sync>,
}

#[component]
fn Frame<'a>(props: &mut FrameProps<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'a>> {
    let theme = hooks.use_context::<Theme>().clone();
    let switch = *hooks.use_context::<ThemeSwitch>();
    let overlay = *hooks.use_context::<OverlayHandle>();
    let mouse = *hooks.use_context::<MouseState>();
    let focus = hooks.use_context::<FocusState>().clone();
    let status = *hooks.use_context::<StatusState>();
    let typing = hooks.use_context::<TypingState>().clone();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let exit_flag = hooks.use_state(|| false);
    if exit_flag.get() {
        system.exit();
    }
    drop(system);

    let quit_message = props.quit_message.clone().unwrap_or_else(|| "Quit?".into());
    let help_entries: Vec<HelpEntry> = builtin_help().into_iter().chain(props.help.clone()).collect();
    let quit = {
        Arc::new(move || {
            let message = quit_message.clone();
            overlay.show(Arc::new(move || {
                let mut exit_flag = exit_flag;
                element! {
                    ConfirmDialog(title: "Quit", message: message.clone(), confirm_label: "Quit".to_string(), on_confirm: move |_| exit_flag.set(true), on_cancel: move |_| overlay.hide())
                }
                .into_any()
            }));
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let help = {
        Arc::new(move || {
            let entries = help_entries.clone();
            overlay.show(Arc::new(move || {
                element! { HelpOverlay(entries: entries.clone(), on_close: move |_| overlay.hide()) }.into_any()
            }));
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let toggle_theme = {
        let name = theme.name.clone();
        Arc::new(move || {
            let next = if name.starts_with("system") { Theme::opencode() } else { Theme::system() };
            status.report(&format!("theme: {}", next.name));
            let mut switch_state = switch.theme;
            switch_state.set(next);
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let toggle_mouse = {
        Arc::new(move || {
            let mut enabled = mouse.enabled;
            let next = !enabled.get();
            enabled.set(next);
            status.report(if next { "mouse: on" } else { "mouse: off" });
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let mut all: Vec<Command> = props.commands.clone();
    all.push(Command { item: ListItem::new("app.theme", "Toggle theme").shortcut("t").section("Application"), run: toggle_theme.clone() });
    all.push(Command { item: ListItem::new("app.mouse", "Toggle mouse").shortcut("m").section("Application"), run: toggle_mouse.clone() });
    all.push(Command { item: ListItem::new("app.help", "Keyboard shortcuts").shortcut("?").section("Application"), run: help.clone() });
    all.push(Command { item: ListItem::new("app.quit", "Quit").shortcut("q").section("Application"), run: quit.clone() });
    let palette = {
        let all = all.clone();
        Arc::new(move || {
            let items: Vec<ListItem> = all.iter().map(|c| c.item.clone()).collect();
            let commands = all.clone();
            overlay.show(Arc::new(move || {
                let commands = commands.clone();
                element! {
                    Palette(title: "Commands", items: items.clone(), on_close: move |_| overlay.hide(), on_pick: move |item: ListItem| {
                        overlay.hide();
                        if let Some(command) = commands.iter().find(|c| c.item.id == item.id) { (command.run)(); }
                    })
                }
                .into_any()
            }));
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let actions = Actions { palette, help, quit, theme: toggle_theme, mouse: toggle_mouse };

    let descends = focus.registry.lock().expect("focus registry").descends(focus.focused.get());
    let mut bindings = vec![
        Binding::new(&["ctrl+p"], { let a = actions.palette.clone(); move || a() }),
        Binding::new(&["tab"], { let f = focus.clone(); move || f.move_by(1, true) }),
        Binding::new(&["shift+tab"], { let f = focus.clone(); move || f.move_by(-1, true) }),
        Binding::new(&["esc"], { let f = focus.clone(); move || f.move_by(-1, false) }),
    ];
    if descends {
        bindings.push(Binding::new(&["enter"], { let f = focus.clone(); move || f.move_by(1, false) }));
    }
    // Letters are gated at key time: a text field editing in the latest frame takes them as text.
    let letter = |keys: &'static [&'static str], action: Arc<dyn Fn() + Send + Sync>| {
        let typing = typing.clone();
        Binding::new(keys, move || if !typing.typing() { action() })
    };
    bindings.push(letter(&["?"], actions.help.clone()));
    bindings.push(letter(&["q"], actions.quit.clone()));
    bindings.push(letter(&["t"], actions.theme.clone()));
    bindings.push(letter(&["m"], actions.mouse.clone()));
    hooks.use_keys(!overlay.is_open(), bindings, None);

    let (columns, rows) = hooks.use_terminal_size();
    let fit = super::fit::ScreenFit::decide(columns, rows, false);
    let clock = chrono_free_clock();
    let last = status.last_action.read().clone();
    let mut left = vec![StatusSegment::new(&format!("{} {}", Glyphs::ON, last), StatusTone::Success)];
    left.extend(props.status.clone());
    let right = vec![
        StatusSegment::new(if mouse.enabled.get() { "mouse" } else { "no mouse" }, if mouse.enabled.get() { StatusTone::Muted } else { StatusTone::Warning }),
        StatusSegment::new(theme.name.trim_end_matches("-dimmed"), StatusTone::Muted),
        StatusSegment::new(&format!("{columns}×{rows}"), StatusTone::Muted),
        StatusSegment::new(&clock, StatusTone::Default),
    ];
    let mut hints = props.hints.clone();
    hints.extend([KeyHint::new("tab", "focus"), KeyHint::new("ctrl+p", "commands"), KeyHint::new("?", "help"), KeyHint::new("q", "quit")]);
    let on_hint = {
        let actions = actions.clone();
        let focus = focus.clone();
        move |hint: KeyHint| match hint.key.as_str() {
            _ if hint.run.is_some() => (hint.run.as_ref().expect("checked"))(),
            "tab" => focus.move_by(1, true),
            "ctrl+p" => (actions.palette)(),
            "?" => (actions.help)(),
            "q" => (actions.quit)(),
            _ => {}
        }
    };
    element! {
        View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_shrink: 1.0_f32, min_height: 0, overflow: Overflow::Hidden) {
            View(flex_direction: FlexDirection::Column, flex_grow: 1.0_f32, flex_shrink: 1.0_f32, min_height: 0, overflow: Overflow::Hidden) {
                #(props.children.iter_mut())
            }
            #(if fit.status_line { Some(element! { StatusLine(left: left, right: right) }) } else { None })
            #(if fit.key_hints { Some(element! {
                View(padding_left: 2, padding_right: 2) {
                    KeyHintBar(left: props.context.clone(), hints: hints, on_press: on_hint)
                }
            }) } else { None })
        }
    }
}

/// HH:MM from the system clock without a date crate.
fn chrono_free_clock() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let local = secs as i64 + local_offset_seconds();
    let minutes = (local / 60) % (24 * 60);
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

/// The local UTC offset in seconds, read from the TZ-aware libc clock.
fn local_offset_seconds() -> i64 {
    // Rust's std has no local time; `date +%z` style parsing keeps us dependency-free.
    std::process::Command::new("date")
        .arg("+%z")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            let s = s.trim();
            let sign = if s.starts_with('-') { -1 } else { 1 };
            let hours: i64 = s.get(1..3)?.parse().ok()?;
            let minutes: i64 = s.get(3..5)?.parse().ok()?;
            Some(sign * (hours * 3600 + minutes * 60))
        })
        .unwrap_or(0)
}
