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
}
