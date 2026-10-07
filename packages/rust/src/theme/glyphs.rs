/// The characters the design system draws states with. Every glyph is East Asian Width N (neutral),
/// so it is one cell in every terminal; A (ambiguous) glyphs such as ● ○ • ■ □ are banned because
/// CJK terminals often draw them two cells wide and break the layout.
pub struct Glyphs;

impl Glyphs {
    /// A binary state that is on: toggles, radio choices, filter chips, status dots.
    pub const ON: &'static str = "▪";
    /// The same state off.
    pub const OFF: &'static str = "▫";
    /// A check box.
    pub const CHECKED: &'static str = "☑";
    pub const UNCHECKED: &'static str = "☐";
    /// A closed and an open tree branch.
    pub const COLLAPSED: &'static str = "▸";
    pub const EXPANDED: &'static str = "▾";
    /// The focus bar and its field-scale twin.
    pub const BAR: &'static str = "┃";
    /// The thin line hugging the active tab label from below.
    pub const OVERLINE: &'static str = "▔";
    /// A half-block bottom edge of a surface, and the bar's foot under a focused input.
    pub const EDGE: &'static str = "▀";
    pub const EDGE_FOOT: &'static str = "╹";
    /// Scrollbar track and thumb.
    pub const TRACK: &'static str = "│";
    pub const THUMB: &'static str = "┃";
    /// The text cursor of a one-line field.
    pub const CURSOR: &'static str = "█";
}
