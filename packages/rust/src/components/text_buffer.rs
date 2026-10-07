/// The lines and cursor of a multi-line editor; every operation returns a new buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextBuffer {
    pub lines: Vec<String>,
    pub row: usize,
    pub col: usize,
}

impl TextBuffer {
    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.split('\n').map(|l| l.to_string()).collect();
        let row = lines.len() - 1;
        let col = lines[row].chars().count();
        Self { lines, row, col }
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    fn chars(&self, row: usize) -> Vec<char> {
        self.lines[row].chars().collect()
    }

    fn with_line(&self, row: usize, chars: Vec<char>) -> Vec<String> {
        let mut lines = self.lines.clone();
        lines[row] = chars.into_iter().collect();
        lines
    }

    pub fn insert(&self, text: &str) -> Self {
        let mut chars = self.chars(self.row);
        let inserted: Vec<char> = text.chars().collect();
        let count = inserted.len();
        chars.splice(self.col..self.col, inserted);
        Self { lines: self.with_line(self.row, chars), row: self.row, col: self.col + count }
    }

    pub fn newline(&self) -> Self {
        let chars = self.chars(self.row);
        let before: String = chars[..self.col].iter().collect();
        let after: String = chars[self.col..].iter().collect();
        let mut lines = self.lines.clone();
        lines.splice(self.row..=self.row, [before, after]);
        Self { lines, row: self.row + 1, col: 0 }
    }

    pub fn backspace(&self) -> Self {
        if self.col > 0 {
            let mut chars = self.chars(self.row);
            chars.remove(self.col - 1);
            return Self { lines: self.with_line(self.row, chars), row: self.row, col: self.col - 1 };
        }
        if self.row == 0 {
            return self.clone();
        }
        let previous = self.chars(self.row - 1);
        let merged = format!("{}{}", self.lines[self.row - 1], self.lines[self.row]);
        let mut lines = self.lines.clone();
        lines.splice(self.row - 1..=self.row, [merged]);
        Self { lines, row: self.row - 1, col: previous.len() }
    }

    pub fn move_by(&self, rows: i32, cols: i32) -> Self {
        let row = (self.row as i32 + rows).clamp(0, self.lines.len() as i32 - 1) as usize;
        let length = self.chars(row).len();
        let col = if rows != 0 { self.col.min(length) } else { (self.col as i32 + cols).clamp(0, length as i32) as usize };
        Self { lines: self.lines.clone(), row, col }
    }

    pub fn home(&self) -> Self {
        Self { lines: self.lines.clone(), row: self.row, col: 0 }
    }

    pub fn end(&self) -> Self {
        Self { lines: self.lines.clone(), row: self.row, col: self.chars(self.row).len() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_at_the_cursor_by_characters() {
        let buffer = TextBuffer::from_text("ab").move_by(0, -1).insert("한🚀");
        assert_eq!(buffer.text(), "a한🚀b");
        assert_eq!(buffer.col, 3);
    }

    #[test]
    fn splits_on_newline_and_joins_on_backspace_at_column_zero() {
        let split = TextBuffer::from_text("hello world").move_by(0, -6).newline();
        assert_eq!(split.lines, vec!["hello", " world"]);
        assert_eq!((split.row, split.col), (1, 0));
        let joined = split.backspace();
        assert_eq!(joined.text(), "hello world");
        assert_eq!((joined.row, joined.col), (0, 5));
    }

    #[test]
    fn clamps_the_column_when_moving_to_a_shorter_line() {
        let buffer = TextBuffer::from_text("long line\nab").move_by(-1, 0).end().move_by(1, 0);
        assert_eq!((buffer.row, buffer.col), (1, 2));
    }
}
