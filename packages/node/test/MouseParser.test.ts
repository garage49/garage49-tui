import {describe, expect, it} from 'vitest';
import {MouseParser} from '../src/input/MouseParser.js';

describe('MouseParser', () => {
  it('decodes a left press and release into 0-based coordinates', () => {
    expect(MouseParser.parse('\x1b[<0;10;5M\x1b[<0;10;5m')).toEqual([
      {kind: 'press', button: 'left', x: 9, y: 4, shift: false, alt: false, ctrl: false},
      {kind: 'release', button: 'left', x: 9, y: 4, shift: false, alt: false, ctrl: false},
    ]);
  });

  it('decodes wheel and modifier bits', () => {
    expect(MouseParser.parse('\x1b[<65;1;1M')[0]).toMatchObject({kind: 'wheel-down', button: 'none'});
    expect(MouseParser.parse('\x1b[<20;1;1M')[0]).toMatchObject({kind: 'press', button: 'left', ctrl: true, shift: true});
  });

  it('recognises a report so key handlers can skip it', () => {
    expect(MouseParser.isReport('\x1b[<0;1;1M')).toBe(true);
    expect(MouseParser.isReport('\x1b[A')).toBe(false);
  });
});
