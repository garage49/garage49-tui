# garage49-tui-iocraft

The garage49 TUI design system for Rust: OpenCode's look and feel as [iocraft](https://github.com/ccbrown/iocraft) components, with an application shell that gives every app the same navigation, focus, mouse, overlays and status bars. It is the Rust twin of `@garage49/garage49-tui-ink`; both implement the same specification.

## Install

```toml
[dependencies]
garage49-tui-iocraft = "0.1"
iocraft = "0.9"
smol = "2"   # for use_future timers (spinners, polling)
```

Requires Rust 1.80 or later.

## Use

```rust
use garage49_tui_iocraft::{run, App, Content, ListItem, Main, MainFocus, Nav, Sidebar, Tab, Table, Column, Row, UseStatus};
use iocraft::prelude::*;

#[component]
fn ProjectsPage(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let focused = hooks.use_context::<MainFocus>().0;
    let status = hooks.use_status();
    let selected = hooks.use_state(|| Some("garage49-tui".to_string()));
    element! {
        Table(columns: vec![Column::new("Name", 24)], rows: vec![Row::new("garage49-tui", &["garage49-tui"])],
            selected_id: selected.read().clone(), focused: focused,
            on_select: move |row: Row| { let mut selected = selected; selected.set(Some(row.id)) },
            on_click: move |row: Row| status.report(&format!("open: {}", row.id)))
    }
}

#[component]
fn MyApp(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let view = hooks.use_state(|| "work".to_string());
    let page = hooks.use_state(|| "projects".to_string());
    let views = vec![Tab::new("work", "Work"), Tab::new("settings", "Settings")];
    let pages = vec![ListItem::new("projects", "Projects").section("Browse")];
    let on_view = move |tab: Tab| { let mut view = view; view.set(tab.id) };
    let on_page = move |item: ListItem| { let mut page = page; page.set(item.id) };
    let with_sidebar = view.read().as_str() == "work";
    element! {
        App(context: "myapp") {
            Nav(brand: "myapp", items: views, active_id: view.read().clone(), on_change: on_view, right: "v1.0.0".to_string())
            Content {
                #(if with_sidebar { Some(element! { Sidebar(items: pages, selected_id: page.read().clone(), on_select: on_page) }) } else { None })
                Main(title: "Projects".to_string()) { ProjectsPage }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    run(element!(MyApp).into_any(), None)
}
```

`run` mounts the app in fullscreen with the theme that fits the terminal (OpenCode's truecolor theme, or the system theme without truecolor) and restores the terminal on exit. `App` provides focus cycling (tab, enter/esc), the mouse, the overlay slot, the command palette (ctrl+p), help (?), the quit confirmation (q), theme (t) and mouse (m) toggles, the status line and the key hint bar.

## Rules of use

Read these before writing app code; each one cost a trial project time.

1. **The root renders `App` and nothing else.** `OverlayHandle`, `use_status`, `use_typing`, `use_focus_region` and `MainFocus` are contexts that `App` provides, so they work only in App's children. Put the app body, the pages and every hook call in child components; the root component only returns `element!(App { … })`.
2. **Running inside tokio (or any other runtime).** `run()` is `smol::block_on(render_loop)`. From a tokio program call it in `tokio::task::spawn_blocking` (or a dedicated thread) and keep the async work on the tokio side. `smol` is still needed for `use_future` timers.
3. **Pushing a background task's state into the UI.** Clone a `State<T>` handle (it is `Copy`) into the task and call `set` from there; `State::set` works from outside the render thread. Alternatively poll a shared handle with `use_future` on a timer. Do not reach into the UI from the task any other way.
4. **Page letter keys yield while a field is editing.** A page that binds letters (`j k e d s …`) must pass `active: focused && !typing` (read `TypingState` through `use_context`) or those keys also type into the editing field. App's own `q ? t m` already yield.
5. **Editor round trips.** The library has no suspend/resume yet. Until it does, stop the render loop (exit the `App`), run the editor, then mount the `App` again; UI state that is not kept outside the tree is lost, so keep it in your model.
6. **Mouse needs a `View` root.** A component that listens to the mouse (`use_mouse`) must have a `View` as its root element; local mouse events do not reach a component whose root is another component.

## Build

```bash
cargo build
```

## Test

Unit tests:

```bash
cargo test
```

Complexity check (clippy: cognitive complexity 10, 50 lines per function, 5 arguments; limits in `clippy.toml`, lints denied in `Cargo.toml`):

```bash
cargo clippy --all-targets
```

## Run

The gallery in `samples/rust` of the repository shows every component:

```bash
cargo run -p garage49-tui-gallery
```
