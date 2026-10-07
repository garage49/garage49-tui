/**
 * The characters the design system draws states with. Every glyph is East Asian Width N (neutral),
 * so it is one cell in every terminal; A (ambiguous) glyphs such as ● ○ • ■ □ are banned because
 * CJK terminals often draw them two cells wide and break the layout.
 */
export class Glyphs {
  /** A binary state that is on: toggles, radio choices, filter chips, status dots. */
  static readonly on = '▪'; // U+25AA
  /** The same state off. */
  static readonly off = '▫'; // U+25AB
  /** A check box. */
  static readonly checked = '☑'; // U+2611
  static readonly unchecked = '☐'; // U+2610
  /** A closed and an open tree branch. */
  static readonly collapsed = '▸'; // U+25B8
  static readonly expanded = '▾'; // U+25BE
  /** The focus bar and its field-scale twin. */
  static readonly bar = '┃'; // U+2503
}
