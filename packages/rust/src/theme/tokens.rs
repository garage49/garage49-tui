use iocraft::Color;

/// Semantic color tokens. Components refer only to these names, never to raw colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeTokens {
    pub text: Color,
    pub text_muted: Color,
    pub text_bright: Color,
    pub accent: Color,
    pub accent_secondary: Color,
    pub heading: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub background: Color,
    pub panel: Color,
    pub surface: Color,
    pub surface_raised: Color,
    pub selection_background: Color,
    pub selection_text: Color,
}

/// A named set of tokens. The same theme can be dimmed for the screen under an overlay.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub name: String,
    pub tokens: ThemeTokens,
}

impl Theme {
    pub fn new(name: &str, tokens: ThemeTokens) -> Self {
        Self { name: name.to_string(), tokens }
    }

    /// OpenCode's default truecolor theme, measured from OpenCode v2.0.21.
    pub fn opencode() -> Self {
        let rgb = |r, g, b| Color::Rgb { r, g, b };
        Self::new(
            "opencode",
            ThemeTokens {
                text: rgb(0xee, 0xee, 0xee),
                text_muted: rgb(0x80, 0x80, 0x80),
                text_bright: rgb(0xff, 0xff, 0xff),
                accent: rgb(0xfa, 0xb2, 0x83),
                accent_secondary: rgb(0x5c, 0x9c, 0xf5),
                heading: rgb(0x9d, 0x7c, 0xd8),
                success: rgb(0x7f, 0xd8, 0x8f), // provisional: from OpenCode's theme file, not measured
                warning: rgb(0xf5, 0xa7, 0x42), // provisional
                error: rgb(0xe0, 0x6c, 0x75),   // provisional
                background: rgb(0x0a, 0x0a, 0x0a),
                panel: rgb(0x14, 0x14, 0x14),
                surface: rgb(0x1e, 0x1e, 0x1e),
                surface_raised: rgb(0x28, 0x28, 0x28),
                selection_background: rgb(0xfa, 0xb2, 0x83),
                selection_text: rgb(0x0a, 0x0a, 0x0a),
            },
        )
    }

    /// Fallback for terminals without truecolor: ANSI hues, grays from the 256-color gray ramp (232…255).
    pub fn system() -> Self {
        Self::new(
            "system",
            ThemeTokens {
                text: Color::AnsiValue(253),
                text_muted: Color::AnsiValue(244),
                text_bright: Color::AnsiValue(231),
                accent: Color::Yellow,
                accent_secondary: Color::Blue,
                heading: Color::Magenta,
                success: Color::Green,
                warning: Color::Yellow,
                error: Color::Red,
                background: Color::AnsiValue(232),
                panel: Color::AnsiValue(233),
                surface: Color::AnsiValue(234),
                surface_raised: Color::AnsiValue(236),
                selection_background: Color::Yellow,
                selection_text: Color::Black,
            },
        )
    }

    /// The same theme with every color pulled towards black, used under an overlay.
    pub fn dimmed(&self, factor: f32) -> Theme {
        let t = &self.tokens;
        let d = |c: Color| Self::scale(c, factor);
        Theme::new(
            &format!("{}-dimmed", self.name),
            ThemeTokens {
                text: d(t.text),
                text_muted: d(t.text_muted),
                text_bright: d(t.text_bright),
                accent: d(t.accent),
                accent_secondary: d(t.accent_secondary),
                heading: d(t.heading),
                success: d(t.success),
                warning: d(t.warning),
                error: d(t.error),
                background: d(t.background),
                panel: d(t.panel),
                surface: d(t.surface),
                surface_raised: d(t.surface_raised),
                selection_background: d(t.selection_background),
                selection_text: d(t.selection_text),
            },
        )
    }

    fn scale(color: Color, factor: f32) -> Color {
        match color {
            Color::Rgb { r, g, b } => {
                let s = |v: u8| (v as f32 * factor).round() as u8;
                Color::Rgb { r: s(r), g: s(g), b: s(b) }
            }
            // the gray ramp 232…255 shrinks towards 232; other 256-color values stay
            Color::AnsiValue(v) if v >= 232 => Color::AnsiValue(232 + ((v - 232) as f32 * factor).round() as u8),
            Color::Black => Color::Black,
            // a named ANSI hue cannot be dimmed; fall back to gray
            _ => Color::DarkGrey,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dims_truecolor_tokens_by_the_factor() {
        let dimmed = Theme::opencode().dimmed(0.4);
        assert_eq!(dimmed.tokens.background, Color::Rgb { r: 4, g: 4, b: 4 });
        assert_eq!(dimmed.tokens.text_muted, Color::Rgb { r: 51, g: 51, b: 51 });
    }

    #[test]
    fn dims_the_gray_ramp_and_turns_hues_gray() {
        let dimmed = Theme::system().dimmed(0.4);
        assert_eq!(dimmed.tokens.text, Color::AnsiValue(240));
        assert_eq!(dimmed.tokens.accent, Color::DarkGrey);
        assert_eq!(dimmed.tokens.selection_text, Color::Black);
    }
}
