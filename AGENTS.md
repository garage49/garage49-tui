# garage49-tui

## Purpose and direction

A TUI design system with the OpenCode look and feel, and the component libraries that implement it in each language the user builds TUIs in. The specification (tokens, color roles, glyphs, surfaces, focus and key rules, layout, standard screens, components with their states and keys, quality bar) lives in this repository; every library implements it, and the user's projects use a library so every TUI looks and behaves the same.

- Users: the user and the agents working in the user's projects. The `tui-design-system` skill in the user's skills repository points projects at this specification and these libraries; it does not duplicate them.
- In scope: the specification, `@garage49/garage49-tui-ink` (Node.js, Ink 8 + React 19) and `garage49-tui-iocraft` (Rust, iocraft 0.9), a gallery per library that shows every element, and pixel-identical rendering between the libraries.
- Out of scope: application logic, themes beyond OpenCode's and the system fallback, terminals without 256 colors as a design target (they still run, degraded).
- Distribution: npm and crates.io packages from this public repository (`garage49/garage49-tui`); the galleries run from the repository.
- Method agreed with the user: build samples first, then extract the library; every change is shown in the gallery and verified in a terminal before it is accepted.

## Baseline

These rules come from the project-baseline skill. A rule that differs here on purpose carries
`<!-- baseline: intentional — REASON -->` right after it.

### Working with the user

- Before building, discuss the project's direction with the user in depth, and record what
  was agreed in "Purpose and direction".
- Understand the intent of every request before working on it. When the intent or the content
  is unclear, keep asking until it is clear. Never fill a gap with a guess.
- This file is maintained with the user. When the user and the agent agree on a decision or a
  standard, record it here in the same change.

### Specification (aterm)

- Write the specification with aterm (use the aterm skill) before implementing what it
  describes, and keep it current when behaviour changes.
- Model the domain in aterm when the project needs it.

### Testing (TDD)

- Develop test-first: write a failing unit test, make it pass with the simplest code, then
  refactor with the tests passing.
- Unit tests are mandatory. Every class has unit tests, and a change without tests is not done.
- A bug fix starts with a unit test that reproduces the bug.

### Coding style

- Write code as classes; in a language without classes, use its equivalent (for example a
  struct with methods). Do not write free functions or global state.
  <!-- baseline: intentional — UI components are functions, as Ink (React) and iocraft require; themes, text width, key chords, layouts (ListLayout, TreeLayout, ScrollThumb), text buffers and registries stay classes. -->
- When behaviour must be shared or varied, use a class pattern (for example a strategy,
  factory or service object).
- Keep the program entry point a single call into a class.

### Complexity

- The complexity checker and its limits are set up and agreed with the user before the first
  line of code, and recorded in "Commands". Changing a limit is the user's decision.
- Work is done only when the unit tests and the complexity check both pass.

### README

- README.md covers how to install, build, test and run the program.

## Commands

| Purpose | Command |
|---|---|
| Specification check | `aterm corpus check` |
| Unit tests (Node) | `pnpm test` |
| Unit tests (Rust) | `cargo test --workspace` |
| Complexity check (Node: ESLint `complexity` 10, `max-lines-per-function` 50, `max-params` 5) | `pnpm complexity` |
| Complexity check (Rust: clippy `cognitive_complexity` 10, `too_many_lines` 50, `too_many_arguments` 5) | `cargo clippy --workspace --all-targets` |
| Everything (Node) | `pnpm check` |
| Gallery (Node) | `pnpm gallery` |
| Gallery (Rust) | `cargo run -p garage49-tui-gallery` |

lizard was tried first and dropped: it skips `.tsx` when scanning and counts destructured props as parameters. The language-native tools parse each language precisely with the same limits.

## Project rules

- Specification: `docs/design-system.trm` (aterm, Knowledge `tui_design_system`) is the source of truth; `docs/DESIGN.md` is its human-readable summary and is updated in the same change. HANDOFF.md is history.
- Package names: npm `@garage49/garage49-tui-ink`, crates.io `garage49-tui-iocraft`. The implementation library's name is part of the package name so another implementation can sit beside it.
- Colors are referred to only through the theme's semantic tokens, never raw values. Color roles: accent (orange) is the cursor, exactly one per focused region; accentSecondary (blue) is the focused region's bar and the editing field's bar, shown only where the keys go; success/warning/error are state values; heading (violet) is section headings and the logo block.
- Themes: `opencode` (truecolor, measured from OpenCode) is the default; `system` is the fallback without truecolor and uses the terminal's ANSI hues with grays from the 256-color ramp (232…255). The screen under an overlay is the same theme dimmed to 0.4.
- Glyphs are East Asian Width N only (`▪ ▫ ☑ ☐ ▸ ▾ ┃ ▔ ▀ ╹ │ █`); ambiguous-width glyphs (● ○ • ■ □ ◉) are banned. All glyphs live in one place per library (`Glyphs`). Display width is measured with string-width / unicode-width, never string length.
- No line borders: regions are surfaces (background colors); a surface's bottom edge is a half block. Binary states use one glyph pair (▪ on / ▫ off); check boxes use ☑/☐.
- Focus: a region keeps its first column free; while focused, a blue ┃ is painted over that column (a region may limit it to `rows`). Tab/Shift+Tab cycle regions in screen order; Enter goes down from Nav and Sidebar, Esc goes up, both stop at the ends; arrows stay inside a region. A click focuses a region. Fields follow the same rule at field scale (FieldBar); a click outside an editing text field ends editing; a bar-only field (Checkbox, RadioGroup) paints its bar column in its container's color, which Section, MasterDetail and Overlay provide (`useSurface` / `RegionSurface` context). A list cursor moves with ↑↓/jk; Space or Enter chooses.
- Layout is composed in the tree: `App > (Nav) + Content > (Sidebar) + Main`; Nav and Sidebar are optional but at least one of them is present; the bottom status line and key hint bar are the App's. Nav is a two-row header: logo block + large tabs with a ▔ under the active one; in-page tabs are small (filled label).
- Indent is 2 (region content) or 4 (inside a section) and every indent step is 2, never 1 (tree depth, options under a heading, a value after its focus bar; a form rounds its label column up to even); selectable rows start where other text starts; nothing above a section title or tabs; sections never nest (a section title is a header bar, a second level is a group heading); a Split holds only Sections, draws nothing of its own and sits directly in the page, so the only visible structure is sections; a list with the detail of its selected row is a MasterDetail inside one section (the selected row's fill runs into the detail), never two sections. A page may open with one Intro (panel block without a header bar, text only: a bright title line and a muted explanation); every Section has a title, so Intro is the only block without a header bar. Pages are composed of an optional Intro, then Section, Split, Form and MasterDetail plus the data and field components; padding, label and value widths come from those, never from hand-set numbers (rhythm: 2×1 padding, 1-cell gap between sections side by side, 1 blank row between sections, 0 between fields; form 60 wide, label column = longest label + 2 ≥ 12, text values ≥ 24).
- Vocabulary is fixed (DESIGN.md "Vocabulary", spec Terms): token, screen, view, page, content, region, focus bar, header bar, group heading, cursor, editing, typing state. State words are never swapped: focused (region), active (view/tab), current (page, field), selected (cursor item), chosen (radio/checkbox value), editing (text field). "Selection" in code (`selectedId`, selection tokens) is the cursor.
- Overlays (dialogs, palette, help, dropdowns) all go through the Screen's one slot; the screen below is dimmed; a click outside hides them; dropdowns are anchored, the rest centered.
- Keys are bindings as data (`useKeys` / `use_keys`), never handler chains; printable text falls through to text fields; while a text field is editing, single-letter app keys (q ? t m) are text.
- Mouse: SGR reporting on by default; `m` toggles it so the terminal's drag-to-select works; clicks bubble innermost → outermost within the top layer; the wheel moves cursors and scrolls views. In iocraft a component that listens to the mouse must have a View as its root.
- No minimum terminal size: the App degrades instead of refusing (below 64 columns a sidebar layout keeps the sidebar alone; below 12 rows the key hint bar goes, below 8 the status line too); a notice below a minimum is an App option, off by default. Spinners advance every 160 ms.
- Builds and releases follow the garage49 build farm rules (github.com/garage49/.github, RULES.md): pnpm workspace, one `.github/workflows/build.yml` calling the shared workflows, releases only from a `vX.Y.Z` tag on `main` whose version equals the root `package.json`, `packages/node/package.json` and `packages/rust/Cargo.toml`; no publish tokens in the repo (trusted publishing).
- Every element is shown in both galleries, and the galleries are compared in a 110×32 tmux pane before a change is accepted.
