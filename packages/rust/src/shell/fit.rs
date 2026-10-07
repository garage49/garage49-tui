/// What the App keeps at a given terminal size: it degrades instead of refusing to draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fit {
    /// Content shows the Main page; false keeps only the Sidebar (narrow terminal with a sidebar).
    pub main: bool,
    pub status_line: bool,
    pub key_hints: bool,
}

/// Decides what fits. Below 64 columns a sidebar-and-main layout keeps the sidebar alone (the
/// navigation stays readable; a mirror pane may own the rest of the screen); below 12 rows the key
/// hint bar goes, below 8 rows the status line too. Nothing is ever refused.
pub struct ScreenFit;

impl ScreenFit {
    pub const NARROW_COLUMNS: u16 = 64;
    pub const SHORT_ROWS: u16 = 12;
    pub const TINY_ROWS: u16 = 8;

    pub fn decide(columns: u16, rows: u16, has_sidebar: bool) -> Fit {
        Fit {
            main: !has_sidebar || columns >= Self::NARROW_COLUMNS,
            status_line: rows >= Self::TINY_ROWS,
            key_hints: rows >= Self::SHORT_ROWS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_everything_at_a_normal_size() {
        assert_eq!(ScreenFit::decide(110, 32, true), Fit { main: true, status_line: true, key_hints: true });
    }

    #[test]
    fn keeps_only_the_sidebar_when_narrow_but_main_without_a_sidebar() {
        assert!(!ScreenFit::decide(60, 40, true).main);
        assert!(ScreenFit::decide(60, 40, false).main);
    }

    #[test]
    fn drops_key_hints_below_12_rows_and_the_status_line_below_8() {
        let short = ScreenFit::decide(110, 11, false);
        assert!(!short.key_hints && short.status_line);
        let tiny = ScreenFit::decide(110, 7, false);
        assert!(!tiny.key_hints && !tiny.status_line);
    }
}
