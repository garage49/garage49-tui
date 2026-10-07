import stringWidth from 'string-width';

/** Display width of text in terminal columns: CJK and emoji count as 2, as Ink's own layout measures them. */
export class TextWidth {
  static of(text: string): number {
    return stringWidth(text);
  }

  static widest(texts: readonly string[]): number {
    return Math.max(0, ...texts.map(text => stringWidth(text)));
  }

  /** The text cut to `width` columns with an ellipsis when it does not fit. */
  static truncate(text: string, width: number): string {
    if (stringWidth(text) <= width) return text;
    if (width <= 1) return width === 1 ? '…' : '';
    let out = '';
    for (const char of text) {
      if (stringWidth(out + char) > width - 1) break;
      out += char;
    }
    return out + '…';
  }
}
