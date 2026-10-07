/** Semantic color tokens. Components refer only to these names, never to raw colors. */
export type ThemeTokens = {
  readonly text: string;
  readonly textMuted: string;
  readonly textBright: string;
  readonly accent: string;
  readonly accentSecondary: string;
  readonly heading: string;
  readonly success: string;
  readonly warning: string;
  readonly error: string;
  readonly background: string;
  readonly panel: string;
  readonly surface: string;
  readonly surfaceRaised: string;
  readonly selectionBackground: string;
  readonly selectionText: string;
};

export class Theme {
  constructor(readonly name: string, readonly tokens: ThemeTokens) {}

  /** The same theme with every color pulled towards black, used under an overlay. */
  dimmed(factor = 0.4): Theme {
    const entries = Object.entries(this.tokens).map(([key, value]) => [key, Theme.scale(value, factor)]);
    return new Theme(`${this.name}-dimmed`, Object.fromEntries(entries) as ThemeTokens);
  }

  private static scale(color: string, factor: number): string {
    if (color.startsWith('#')) {
      const channels = [1, 3, 5].map(i => Math.round(parseInt(color.slice(i, i + 2), 16) * factor));
      return '#' + channels.map(c => c.toString(16).padStart(2, '0')).join('');
    }
    const gray = /^ansi256\((\d+)\)$/.exec(color);
    if (gray) {
      const index = Number(gray[1]);
      if (index >= 232) return `ansi256(${232 + Math.round((index - 232) * factor)})`; // the gray ramp 232…255
      return color;
    }
    return color === 'black' ? color : 'gray'; // a named ANSI hue cannot be dimmed; fall back to gray
  }
}

/** OpenCode's default truecolor theme, measured from OpenCode v2.0.21. */
export const opencodeTheme = new Theme('opencode', {
  text: '#eeeeee',
  textMuted: '#808080',
  textBright: '#ffffff',
  accent: '#fab283',
  accentSecondary: '#5c9cf5',
  heading: '#9d7cd8',
  success: '#7fd88f', // provisional: taken from OpenCode's theme file, not measured
  warning: '#f5a742', // provisional
  error: '#e06c75', // provisional
  background: '#0a0a0a',
  panel: '#141414',
  surface: '#1e1e1e',
  surfaceRaised: '#282828',
  selectionBackground: '#fab283',
  selectionText: '#0a0a0a',
});

/**
 * Fallback for terminals without truecolor. Hues are the terminal's own ANSI colors; the gray
 * levels come from the 256-color gray ramp (232…255), which every non-truecolor terminal of the
 * last decades supports. 16 ANSI colors alone have only two dark grays, so the surfaces and the
 * muted text could not be told apart. On a 16-color terminal chalk picks the nearest color.
 */
export const systemTheme = new Theme('system', {
  text: 'ansi256(253)', // #dadada
  textMuted: 'ansi256(244)', // #808080
  textBright: 'ansi256(231)', // #ffffff
  accent: 'yellow',
  accentSecondary: 'blue',
  heading: 'magenta',
  success: 'green',
  warning: 'yellowBright',
  error: 'red',
  background: 'ansi256(232)', // #080808
  panel: 'ansi256(233)', // #121212
  surface: 'ansi256(234)', // #1c1c1c
  surfaceRaised: 'ansi256(236)', // #303030
  selectionBackground: 'yellow',
  selectionText: 'black',
});
