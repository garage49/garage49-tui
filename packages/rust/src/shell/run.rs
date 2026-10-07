use iocraft::prelude::*;

use crate::components::ThemeRoot;
use crate::theme::Theme;

/// Picks the theme for this terminal: OpenCode's truecolor theme when available, the system theme otherwise.
pub struct TerminalTheme;

impl TerminalTheme {
    pub fn detect() -> Theme {
        let colorterm = std::env::var("COLORTERM").unwrap_or_default();
        let truecolor = colorterm == "truecolor" || colorterm == "24bit";
        if std::env::var("G49_THEME").as_deref() == Ok("system") || !truecolor {
            Theme::system()
        } else {
            Theme::opencode()
        }
    }
}

/// Mounts an App in fullscreen with the detected theme; ctrl+c exits and restores the terminal.
pub fn run(app: AnyElement<'static>, theme: Option<Theme>) -> std::io::Result<()> {
    let theme = theme.unwrap_or_else(TerminalTheme::detect);
    let mut root = element! {
        ThemeRoot(theme: theme) { #(std::iter::once(app)) }
    };
    smol::block_on(root.render_loop().fullscreen().enable_mouse_capture())
}
