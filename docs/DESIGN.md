# garage49 TUI design system

The readable summary of `docs/design-system.trm`. The specification is the source of truth; this page restates it for people. Both libraries, `@garage49/garage49-tui-ink` and `garage49-tui-iocraft`, implement every rule here and render the same screen for the same tree.

## Look and feel

OpenCode's, adopted in full and measured from OpenCode v2.0.21:

- **No line borders.** Regions are surfaces, told apart by background color. A surface's bottom edge is a half block (`▀`), as under OpenCode's input box.
- **Typography.** Bold for titles, section headings and the selected row; muted for secondary text; no italics.
- **Pop-ups** are centered panels without a border: bold title at the left, `esc` muted at the right. The screen behind is dimmed.
- **Selection** fills the whole row with the accent color and turns the text to the background color; shortcuts sit right-aligned and muted.

## Themes and tokens

Components refer only to semantic tokens, never to raw colors.

| Token | opencode | system (fallback) | Used for |
|---|---|---|---|
| text | `#eeeeee` | ansi256(253) | body text, key names |
| textMuted | `#808080` | ansi256(244) | hints, placeholders, shortcuts |
| textBright | `#ffffff` | ansi256(231) | titles |
| accent | `#fab283` | yellow | the cursor (see color roles) |
| accentSecondary | `#5c9cf5` | blue | the focus bar |
| heading | `#9d7cd8` | magenta | section headings, the logo block |
| success / warning / error | `#7fd88f` / `#f5a742` / `#e06c75` (provisional, from OpenCode's theme file) | green / yellow / red | state values |
| background | `#0a0a0a` | ansi256(232) | the screen |
| panel | `#141414` | ansi256(233) | sidebar, overlay panels, status line |
| surface | `#1e1e1e` | ansi256(234) | inputs, header label row, dropdowns |
| surfaceRaised | `#282828` | ansi256(236) | unfocused selection, chips, buttons |
| selectionBackground / selectionText | `#fab283` / `#0a0a0a` | yellow / black | the cursor row |

`opencode` is the default when the terminal reports truecolor; otherwise `system`, whose hues are the terminal's ANSI colors and whose grays come from the 256-color ramp (16 ANSI colors cannot tell four grays apart). Under an overlay the whole screen below is the same theme **dimmed to 0.4** (truecolor channels and the gray ramp scale; ANSI hues become gray).

## Color roles

- **accent (orange) = the cursor.** Exactly one per focused region: the selected list row, table row, tree row, radio option, chip, button, the active tab's line. When the region loses focus the cursor stays as a raised-surface fill.
- **accentSecondary (blue) = where the keys go.** The focused region's bar, the editing field's bar, the input box's bar and its foot (`╹`). Never used for states.
- **success / warning / error = values.** A toggle that is on, a chosen radio, a checked box, a running status, a warning chip, an error dialog title.
- **heading (violet)** marks section headings in lists and the logo block.

## Glyphs

Every glyph is East Asian Width N (neutral), one cell in every terminal. Ambiguous-width glyphs (`● ○ • ■ □ ◉`) are banned because CJK terminals often draw them two cells wide.

| Role | Glyph |
|---|---|
| binary state on / off (toggle, radio, filter chip, status dot, idle spinner) | `▪` / `▫` |
| check box checked / unchecked | `☑` / `☐` |
| tree branch collapsed / expanded | `▸` / `▾` |
| focus bar, field bar, scrollbar thumb | `┃` |
| active large tab | `▔` under the label |
| surface bottom edge, focused input foot | `▀`, `╹` |
| scrollbar track, one-line cursor | `│`, `█` |

Display width is always measured (string-width / unicode-width), never taken from string length.

## Focus

A **region** is a focusable area (Nav, Sidebar, Main; also a field inside a form). A region keeps its first column free. While it owns the keyboard a blue `┃` is painted over that column for the region's height (a header limits it to its title row); otherwise nothing is drawn, so the region looks exactly as it did. Exactly one blue bar is on screen.

- `tab` / `shift+tab` cycle regions in screen order (top to bottom, then left to right) and wrap.
- `enter` goes down from Nav or Sidebar to the next region; `esc` goes back up; neither wraps.
- Arrows and `hjkl` stay inside a region. Fields leave at their edges (↑ on the first line of a text area, ↓ on the last; the same for radio options).
- A click on a region focuses it; a click on a row also selects the row (events bubble innermost → outermost).

## Keys

Keys are bindings as data: `[{keys: ['up', 'k'], run}, …]`. Chord names: `up down left right pageup pagedown home end enter esc tab shift+tab backspace delete space`, `ctrl+x`, `alt+x`, or the printable character. Printable keys no binding claims go to the editing text field. While a text field is editing, the app's single-letter keys are text.

App keys: `ctrl+p` palette · `?` help · `q` quit (confirmation) · `t` toggle theme · `m` toggle mouse. Lists: `↑↓ jk` move, `enter` activate, `space` choose/toggle. Selects: `enter/space` open, `←→` cycle. Logs: `↑↓ jk` scroll, `pageup/pagedown`, `G`/`end` follow.

## Mouse

SGR reporting is on by default. Clicks select rows, tabs, chips, buttons and key hints and focus regions; the wheel moves list cursors and scrolls logs and text areas. `m` turns reporting off so the terminal's drag-to-select works again (shift+drag works while it is on in most terminals). While an overlay is open only the overlay reacts; a click outside it closes it.

## Layout

An app is composed as a tree; everything else is a default:

```
App                       screen, theme, mouse, overlay slot, focus cycling, status line, key hint bar
├─ (Nav)                  two rows: logo block + large tabs (▔ under the active one) + context at the right
└─ Content                a row
   ├─ Sidebar (optional)  panel with a sectioned list, width 26 by default (a prop)
   └─ Main                page title + page; takes the remaining width
```

Nav and Sidebar are each optional, but at least one of them must be present. The three agreed layouts are the three trees: `App > Nav + Content > Main` (top navigation only), `App > Content > Sidebar + Main` (sidebar only) and `App > Nav + Content > Sidebar + Main` (both). The status line shows `▪ last action · app segments ‖ mouse · theme · columns×rows · clock`; the key hint bar shows muted context at the left and `key label` pairs at the right (clickable).

## Overlays

Every overlay goes through the Screen's single slot: confirm dialog, message dialog, help, command palette, dropdown (select). The screen below is dimmed; a click outside hides it; dropdowns are anchored under their field, the others centered. Dialog buttons: the chosen one is the cursor; `←→`/`tab` move, `enter` confirms, `y`/`n` shortcut, `esc` cancels. A destructive confirmation shows its title in the error color.

## Components

| Component | What it shows | Keys / mouse |
|---|---|---|
| Label | text in one role (default, muted, bright, heading, accent, success, warning, error) | — |
| List | rows under bold violet section headings; shortcut (muted) or value (text) right-aligned; cursor row filled | click selects, wheel moves |
| Table | fixed-width columns, muted bold header, right-aligned numbers, cursor row filled | same as List |
| TreeView | indented rows with ▸/▾ | ↑↓ move, → expand or step in, ← collapse or go to parent, enter/space toggle, click selects and toggles |
| LogView | time (muted) · level (DEBUG muted, INFO blue, WARN warning, ERROR error) · message; newest at the bottom; scrollbar | wheel and ↑↓ scroll, G follows |
| Scrollbar | one column: faint track, thumb sized by the visible share | — |
| Tabs | small: one row, active label filled · large: two rows, ▔ under the active label | ←→ hl, click |
| TopNav | logo block (violet) + large tabs + right text on the surface row | as Tabs; focus bar on the logo's first column |
| StatusLine, KeyHintBar | panel row with segments · context + key/label pairs | hints clickable |
| TextField | label · field bar · value on a surface, `█` cursor | typing, backspace |
| TextArea | several rows, inverted-cell cursor, scrollbar | arrows, enter splits, backspace joins, pageup/pagedown, home/end, wheel scrolls, edge leave |
| Input | OpenCode's input box: bar column, surface, half-block edge, optional footer | typing, enter submits |
| Select | label · field bar · value ▾ | enter/space opens the dropdown, ←→ cycle, click opens |
| Toggle | `▪ on` (success) / `▫ off` (muted) on a surface | space/enter/click toggle |
| Checkbox | `☑` (success) / `☐` (muted) + label | space/enter/click toggle |
| RadioGroup | one row per option, `▪` for the chosen (success), cursor row filled | ↑↓ move the cursor, space/enter choose, click chooses, leaves at the edges |
| Button | label on a raised surface; the chosen one filled | click |
| Chip | small pill on a raised surface: tag (× removes), filter (▪/▫), status tone | click, × |
| Spinner | braille/line/bounce frames in blue, 160 ms, `▪` when idle | — |
| Panel, FocusRegion, FieldBar | the surfaces and the focus bars described above | — |
| Overlay, ConfirmDialog, MessageDialog, HelpOverlay, Palette, Dropdown | the overlays described above | esc closes; palette types to filter |

## Quality bar

- Terminal restored on exit, panic and ctrl+c (raw mode, alternate screen, cursor, mouse reporting).
- Resize handled; below 80×24 the screen shows a notice instead of a broken layout.
- Works under SSH, tmux and Herdr.
- Correct CJK and emoji width: no broken wrapping or truncation.
- Both galleries render the same screen in a 110×32 tmux pane; the comparison is part of accepting a change.
