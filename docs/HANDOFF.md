> History. This brief is kept as the record of the handoff on 2026-10-06. Decisions taken since then live in `AGENTS.md` (rules) and `docs/DESIGN.md` / `docs/design-system.trm` (the specification); where they disagree with this file, they win.

# Handoff: garage49-tui

Written by the `skills` agent (shared-skill development) on 2026-10-06 for the agent that owns
this repository. Everything here was agreed with the user; open questions are marked.

## What this project is

A TUI design system with the OpenCode look and feel, and component libraries that implement it
in each language the user builds TUIs in: Node.js (Ink) and Rust (iocraft) first. A language
without a supported library gets the same library written for it. The user's projects then use
these libraries so every TUI looks and behaves the same.

Plan agreed with the user: **build samples first, then extract the library from them**, publish
from this public repository (`garage49/garage49-tui`). A shared skill `tui-design-system` in the
user's skills repository will point projects at this library once it exists; the `skills` agent
writes that skill, not this project.

## Decisions (agreed)

| Topic | Decision |
|---|---|
| Node.js library | **Ink** (v8, React 19). Chosen over OpenTUI by the user. |
| Rust library | **iocraft** (0.9). Same model as Ink (components, hooks, flexbox via taffy), so one design system maps onto both. ratatui was the alternative; the user chose iocraft after comparing. |
| Look and feel | **OpenCode's**, adopted in full (analysis below). Earlier ideas (line borders, bold-border focus) are dropped. |
| Color | **OpenCode's truecolor theme as the default**, exposed only as semantic tokens; **ANSI 16-color "system" theme as fallback** when truecolor is unavailable or configured; `NO_COLOR` leaves text attributes only. |
| Default layout | Top menu, left sidebar, or both. Without a specific request, pick one of these three. |
| Standard screens | Home, Settings, Help overlay (`?`), confirmation and error dialogs. |
| Quality bar | Terminal restore on exit/panic/Ctrl+C (raw mode, alternate screen, cursor); resize handling and a minimum-size notice; works under SSH, tmux and Herdr; correct CJK/emoji width (no broken wrapping or truncation); key scheme (arrows and hjkl, `q` quit, `Esc` back, `/` search, `?` help) with a key hint bar; mouse optional. |
| project-baseline | Applied (AGENTS.md + CLAUDE.md link, TDD, complexity gate, README). The class rule is excepted for UI components only, marked `<!-- baseline: intentional — … -->`; themes, width and layout calculations stay classes. |

## Components the design system must cover

top menu, sidebar, pane, pop-up (dialog/palette), list, table, input, label, **tab** (added by the
user), plus the key hint bar and status line seen in OpenCode. Add what the samples show is
missing.

## OpenCode look and feel (measured from OpenCode v2.0.21 in a 110×32 tmux pane)

Raw captures with ANSI codes: `docs/reference/opencode-home.ansi`, `docs/reference/opencode-palette.ansi`.

| Element | How OpenCode does it |
|---|---|
| Region separation | **No line borders.** Regions are surfaces with different background colors; edges use half blocks (`▀`, `▄`). |
| Focus / input | A left accent bar: `┃` on each row, `╹` at the bottom; the input surface sits right of it. |
| Pop-up | Centered panel without a border. Title bold at the left, `esc` muted at the right. The screen behind is dimmed (its colors drop to near-background values). |
| List selection | The whole row is filled with the accent background; text turns to the background color. Shortcuts right-aligned, muted. |
| Section headings in lists | Bold, colored (violet). |
| Bottom line | Path muted; key hints as "key (bright) + label (muted)" pairs. |
| Typography | Bold for titles and the selected row; muted for secondary text; no italics seen. |

Measured palette (truecolor):

| Token (proposed) | Value | Seen as |
|---|---|---|
| text | `#eeeeee` | body text, key names |
| textMuted | `#808080` | hints, placeholders, paths, shortcuts |
| textBright | `#ffffff` | emphasis |
| accent (primary) | `#fab283` | `/update` command highlight |
| accentSecondary | `#5c9cf5` | input accent bar, "Build" mode label |
| heading | `#9d7cd8` | list section headings ("Suggested", "Session") |
| background | `#0a0a0a` | screen |
| surface | `#1e1e1e` | input area |
| surfaceRaised | `#282828` / `#434343` | panels, selection shades |
| dimmed text (under a pop-up) | `#353535`, `#26405f` | the home screen while the palette is open |

OpenCode also ships a "system" theme that uses the terminal's own ANSI colors; our fallback
mirrors that idea. Check OpenCode's theme files for the exact semantic names before fixing ours:
<https://github.com/anomalyco/opentui> (library) and the OpenCode repository's `themes/` directory.

## Library probes (both work)

`docs/reference/pane-probe-ink.mjs` and `docs/reference/pane-probe-iocraft.rs` render the same
40-column two-pane screen in Ink and iocraft, including a Korean label; both produce identical
output. They used line borders (now dropped) but prove the layout mechanics:

- A titled header row is "fixed text (flex-shrink 0) + fill (flex-grow 1, flex-basis 0, overflow
  hidden, no-wrap) + fixed text". In Ink the fill is `wrap: 'wrap'` inside `overflow: 'hidden'`
  (`truncate-end` adds an ellipsis); in iocraft it is `TextWrap::NoWrap` with
  `flex_basis: FlexBasis::Length(0)` (`flex_basis` does not accept `0` or `0pct`).
- iocraft 0.9.1 has `background_color`, `border_edges`, `Overflow::Hidden`, RGB colors, `Weight`,
  `TextDecoration::Underline`, `invert`, and honours `NO_COLOR`; it has no built-in border title.
- Ink 8 has `backgroundColor`, `borderTop/Bottom/Left/Right`, `overflow: 'hidden'`, RGB colors;
  no border title either. `ink-testing-library` 4 renders frames for snapshot tests.
- Versions probed: ink 8.0.0, react 19.3.0, ink-testing-library 4.0.0, string-width 8.3.0 (Ink's
  width dependency); iocraft 0.9.1 (uses unicode-width 0.1). Rust 1.99 stable on this host.

## Suggested order

1. Set up the repository per project-baseline (discuss direction with the user first; much of
   it is above, confirm the rest).
2. Samples: the Home screen with sidebar, the command palette pop-up, a Settings form, and a
   table, each in Ink and iocraft, pixel-identical in a tmux pane of the same size.
3. Extract tokens → theme → components into `packages/node` and `packages/rust`.
4. Snapshot tests per component in both languages; a CJK width test; a terminal-restore test.
5. Tell the `skills` agent (`hc send mldev:default/skills`) when the library has a usable API,
   so the `tui-design-system` skill can be written against it.

## Open questions for the user

- Package names for npm and crates.io (`@garage49/tui`? `garage49-tui`?).
- Whether the dimmed-background effect under pop-ups is required in v1 (it needs every
  component to accept a "dimmed" state).
- Minimum terminal size to support.
