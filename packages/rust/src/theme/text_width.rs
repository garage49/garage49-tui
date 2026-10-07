use unicode_width::UnicodeWidthStr;

/// Display width of text in terminal columns: CJK and emoji count as 2, as iocraft's own layout measures them.
pub struct TextWidth;

impl TextWidth {
    pub fn of(text: &str) -> usize {
        UnicodeWidthStr::width(text)
    }

    pub fn widest<'a>(texts: impl IntoIterator<Item = &'a str>) -> usize {
        texts.into_iter().map(Self::of).max().unwrap_or(0)
    }

    /// The text cut to `width` columns with an ellipsis when it does not fit.
    pub fn truncate(text: &str, width: usize) -> String {
        if Self::of(text) <= width {
            return text.to_string();
        }
        if width <= 1 {
            return if width == 1 { "…".to_string() } else { String::new() };
        }
        let mut out = String::new();
        for ch in text.chars() {
            let mut candidate = out.clone();
            candidate.push(ch);
            if Self::of(&candidate) > width - 1 {
                break;
            }
            out = candidate;
        }
        out + "…"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_cjk_and_emoji_as_two_columns() {
        assert_eq!(TextWidth::of("기록"), 4);
        assert_eq!(TextWidth::of("Files"), 5);
        assert_eq!(TextWidth::of("🚀"), 2);
    }

    #[test]
    fn finds_the_widest_by_display_width() {
        assert_eq!(TextWidth::widest(["abc", "기록"]), 4);
    }

    #[test]
    fn truncates_by_display_width_with_an_ellipsis() {
        assert_eq!(TextWidth::truncate("abcdef", 10), "abcdef");
        assert_eq!(TextWidth::truncate("abcdef", 4), "abc…");
        assert_eq!(TextWidth::truncate("한글입니다", 5), "한글…");
        assert_eq!(TextWidth::truncate("abc", 0), "");
    }
}
