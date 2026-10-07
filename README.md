# garage49-tui

A TUI design system with OpenCode's look and feel, and the libraries that implement it:

- `packages/node` — `@garage49/garage49-tui-ink` for Node.js (Ink 8, React 19)
- `packages/rust` — `garage49-tui-iocraft` for Rust (iocraft 0.9)

The specification is `docs/design-system.trm` (aterm); `docs/DESIGN.md` is its readable summary. Each package has its own README with install and usage. `samples/node` and `samples/rust` are the galleries: every element the system defines, one page each, rendered identically by both libraries.

## Install

```bash
pnpm install         # Node workspace (packages/node, samples/node); corepack enable gives you pnpm
cargo build          # Rust workspace (packages/rust, samples/rust)
```

Requires Node.js 20+ with pnpm, Rust 1.80+, and for the specification the `aterm` CLI.

## Build

```bash
pnpm build           # emits packages/node/dist
cargo build --workspace
```

## Test

Unit tests:

```bash
pnpm test
cargo test --workspace
```

Complexity check (same limits in both languages: complexity 10, 50 lines per function, 5 parameters):

```bash
pnpm complexity                                  # ESLint
cargo clippy --workspace --all-targets           # clippy
```

`pnpm check` runs the Node type check, complexity check, tests and build in one go.

Specification check:

```bash
aterm corpus check
```

## Run

```bash
pnpm gallery                         # Node gallery (builds the package first)
cargo run -p garage49-tui-gallery    # Rust gallery
```

Releases: bump `version` in the root `package.json`, `packages/node/package.json` and `packages/rust/Cargo.toml` in one commit on `main`, then tag it `vX.Y.Z`; the garage49 build farm publishes to npm and crates.io from the tag.

Both galleries need a terminal of at least 80×24; truecolor selects the OpenCode theme, otherwise the system theme is used. `G49_THEME=system` forces the fallback.
