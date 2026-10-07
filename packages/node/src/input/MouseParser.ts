export type MouseButton = 'left' | 'middle' | 'right' | 'none';

export type MouseEvent = {
  readonly kind: 'press' | 'release' | 'drag' | 'move' | 'wheel-up' | 'wheel-down';
  readonly button: MouseButton;
  /** 0-based terminal column and row. */
  readonly x: number;
  readonly y: number;
  readonly shift: boolean;
  readonly alt: boolean;
  readonly ctrl: boolean;
};

/** Decodes SGR (1006) mouse reports: ESC [ < button ; column ; row (M|m). */
export class MouseParser {
  private static readonly pattern = /\x1b\[<(\d+);(\d+);(\d+)([Mm])/g;

  static isReport(input: string): boolean {
    return input.startsWith('\x1b[<');
  }

  static parse(chunk: string): MouseEvent[] {
    const events: MouseEvent[] = [];
    for (const match of chunk.matchAll(MouseParser.pattern)) {
      const code = Number(match[1]);
      const x = Number(match[2]) - 1;
      const y = Number(match[3]) - 1;
      const release = match[4] === 'm';
      events.push(MouseParser.decode(code, x, y, release));
    }
    return events;
  }

  private static decode(code: number, x: number, y: number, release: boolean): MouseEvent {
    const modifiers = {shift: (code & 4) !== 0, alt: (code & 8) !== 0, ctrl: (code & 16) !== 0};
    const low = code & 3;
    const button: MouseButton = code >= 64 ? 'none' : low === 0 ? 'left' : low === 1 ? 'middle' : low === 2 ? 'right' : 'none';
    if (code >= 64) return {kind: code === 64 ? 'wheel-up' : 'wheel-down', button, x, y, ...modifiers};
    if (code & 32) return {kind: button === 'none' ? 'move' : 'drag', button, x, y, ...modifiers};
    return {kind: release ? 'release' : 'press', button, x, y, ...modifiers};
  }
}
