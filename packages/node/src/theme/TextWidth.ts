import stringWidth from 'string-width';

/** Display width of text in terminal columns: CJK and emoji count as 2, as Ink's own layout measures them. */
export class TextWidth {
  static of(text: string): number {
    return stringWidth(text);
  }

  static widest(texts: readonly string[]): number {
    return Math.max(0, ...texts.map(text => stringWidth(text)));
  }
}
