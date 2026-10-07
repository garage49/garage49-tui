# Backlog

Requests from projects that adopted the design system, in the order they were reported. Each one is
a gap in both libraries unless a language is named. The user decides the order; the first five came
up in both the herdr-connect and the herdr-ranch conversions.

| # | Request | Where it hurt | Status |
|---|---|---|---|
| 1 | Quick key presses coalesce: key bindings capture render-time values, so two keys in one frame (key repeat, paste) act once | both conversions, every list and text field | done: text fields, text area, input, palette, dropdown, radio and the sidebar chain on the latest value |
| 2 | A scrolling container for whole pages (height measured by the library, not computed by the page); LogView and TextArea should take the remaining height | both (rows = terminal rows − 8 hacks) | open |
| 3 | Table cell and row tones (success/warning/error on a column) | herdr-connect Status/Input columns | open |
| 4 | App options: turn off or rebind the built-in q/t/m, initial focus (start on Main for a read-only screen). The minimum size is done: no gate by default, the App degrades, `minColumns`/`min_columns` is an option | both | partly done |
| 5 | Custom key hints with `on_press` (App's `hints` are display-only) | both | done: a KeyHint carries `run` / `with_action` |
| 6 | ANSI-coloured text block (status command output loses its colours) | herdr-ranch plugin status | open |
| 7 | TextField secret mode (masked value) | herdr-ranch env values | done: `secret` prop shows * per character |
| 8 | Button disabled state (with a reason) | herdr-ranch save/set buttons | open |
| 9 | List: section headings wrap in a narrow list while labels truncate | herdr-ranch sidebar | open |
| 10 | StatusLine: a long left side runs into the right segments | both | done: the left side is cut by display width with … |
| 11 | HelpOverlay: long entries are cut at the right instead of wrapping | herdr-connect | open |
| 12 | Mouse drag (pane border resize) | herdr-ranch | open |
| 13 | Suspend/resume around an external program (editor round trip) | herdr-ranch | open |
| 14 | Nav without tabs (single-view app); Nav right slot with a tone | herdr-connect, herdr-ranch | open |
| 15 | LogView without the level column | herdr-ranch | open |
| 16 | Test render harness (`render_for_test(element, cols, rows)`, Rust) and a public `restore_terminal()` | herdr-connect | open |
| 17 | Tree-shaped sidebar (collapsible sections) | herdr-ranch | open |
| 18 | `.trm`/DESIGN.md: the ambiguous-width ban conflicts with projects that spelled `••••`; document `****` as the masked value | herdr-ranch | done: TextField `secret` shows `*` |
| 19 | A text field unmounted while editing left the app in typing mode (Rust: no unmount hook) | adoc-hub | done: typing is keyed to the App's frame |
