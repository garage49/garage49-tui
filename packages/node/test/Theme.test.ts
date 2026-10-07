import {describe, expect, it} from 'vitest';
import {opencodeTheme, systemTheme} from '../src/theme/Theme.js';

describe('Theme', () => {
  it('dims every truecolor token towards black by the factor', () => {
    const dimmed = opencodeTheme.dimmed(0.4);
    expect(dimmed.tokens.background).toBe('#040404');
    expect(dimmed.tokens.textMuted).toBe('#333333');
    expect(dimmed.tokens.accentSecondary).toBe('#253e62');
  });

  it('dims the 256-color gray ramp and turns ANSI hues gray', () => {
    const dimmed = systemTheme.dimmed(0.4);
    expect(dimmed.tokens.text).toBe('ansi256(240)');
    expect(dimmed.tokens.background).toBe('ansi256(232)');
    expect(dimmed.tokens.accent).toBe('gray');
    expect(dimmed.tokens.selectionText).toBe('black');
  });
});
