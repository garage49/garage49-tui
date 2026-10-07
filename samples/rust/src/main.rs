//! The gallery: every element of the design system, one page each, built on the shell.
mod gallery;
mod pages;

use garage49_tui_iocraft::run;
use iocraft::prelude::*;

fn main() -> std::io::Result<()> {
    run(element!(gallery::Gallery).into_any(), None)
}
