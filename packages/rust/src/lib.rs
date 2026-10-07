//! The garage49 TUI design system for Rust: OpenCode's look and feel as iocraft components,
//! with an application shell that gives every app the same navigation, focus, mouse, overlays and status bars.

pub mod components;
pub mod input;
pub mod shell;
pub mod theme;

pub use components::*;
pub use input::*;
pub use shell::*;
pub use theme::*;
