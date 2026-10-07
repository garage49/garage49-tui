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

describe('TextWidth.truncate', () => {
  it('cuts to the width with an ellipsis, by display width', () => {
    expect(TextWidth.truncate('abcdef', 10)).toBe('abcdef');
    expect(TextWidth.truncate('abcdef', 4)).toBe('abc…');
    expect(TextWidth.truncate('한글입니다', 5)).toBe('한글…');
    expect(TextWidth.truncate('abc', 0)).toBe('');
  });
});
