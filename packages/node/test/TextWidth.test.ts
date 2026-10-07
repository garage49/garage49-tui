import {describe, expect, it} from 'vitest';
import {TextWidth} from '../src/theme/TextWidth.js';

describe('TextWidth', () => {
  it('counts CJK and emoji as two columns', () => {
    expect(TextWidth.of('기록')).toBe(4);
    expect(TextWidth.of('Files')).toBe(5);
    expect(TextWidth.of('🚀')).toBe(2);
  });

  it('finds the widest text by display width, not string length', () => {
    expect(TextWidth.widest(['abc', '기록'])).toBe(4);
  });
});
