/** The lines and cursor of a multi-line editor; every operation returns a new buffer. */
export class TextBuffer {
  constructor(readonly lines: readonly string[], readonly row: number, readonly col: number) {}

  static from(text: string): TextBuffer {
    const lines = text.split('\n');
    const last = lines.length - 1;
    return new TextBuffer(lines, last, [...lines[last]!].length);
  }

  get text(): string {
    return this.lines.join('\n');
  }

  private line(row = this.row): string[] {
    return [...this.lines[row]!];
  }

  private replace(row: number, chars: string[]): string[] {
    return this.lines.map((line, index) => (index === row ? chars.join('') : line));
  }

  insert(text: string): TextBuffer {
    const chars = this.line();
    chars.splice(this.col, 0, ...text);
    return new TextBuffer(this.replace(this.row, chars), this.row, this.col + [...text].length);
  }

  newline(): TextBuffer {
    const chars = this.line();
    const before = chars.slice(0, this.col).join('');
    const after = chars.slice(this.col).join('');
    const lines = [...this.lines.slice(0, this.row), before, after, ...this.lines.slice(this.row + 1)];
    return new TextBuffer(lines, this.row + 1, 0);
  }

  backspace(): TextBuffer {
    if (this.col > 0) {
      const chars = this.line();
      chars.splice(this.col - 1, 1);
      return new TextBuffer(this.replace(this.row, chars), this.row, this.col - 1);
    }
    if (this.row === 0) return this;
    const previous = this.line(this.row - 1);
    const merged = previous.join('') + this.lines[this.row]!;
    const lines = [...this.lines.slice(0, this.row - 1), merged, ...this.lines.slice(this.row + 1)];
    return new TextBuffer(lines, this.row - 1, previous.length);
  }

  move(rows: number, cols: number): TextBuffer {
    const row = Math.min(this.lines.length - 1, Math.max(0, this.row + rows));
    const length = this.line(row).length;
    const col = rows !== 0 ? Math.min(this.col, length) : Math.min(length, Math.max(0, this.col + cols));
    return new TextBuffer(this.lines, row, col);
  }

  home(): TextBuffer {
    return new TextBuffer(this.lines, this.row, 0);
  }

  end(): TextBuffer {
    return new TextBuffer(this.lines, this.row, this.line().length);
  }
}
