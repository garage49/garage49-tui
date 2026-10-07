import {describe, expect, it} from 'vitest';
import {TextBuffer} from '../src/components/TextBuffer.js';

describe('TextBuffer', () => {
  it('inserts at the cursor and advances by characters, not code units', () => {
    const buffer = TextBuffer.from('ab').move(0, -1).insert('한🚀');
    expect(buffer.text).toBe('a한🚀b');
    expect(buffer.col).toBe(3);
  });

  it('splits a line on newline and joins it back on backspace at column 0', () => {
    const split = TextBuffer.from('hello world').move(0, -6).newline();
    expect(split.lines).toEqual(['hello', ' world']);
    expect([split.row, split.col]).toEqual([1, 0]);
    const joined = split.backspace();
    expect(joined.text).toBe('hello world');
    expect([joined.row, joined.col]).toEqual([0, 5]);
  });

  it('clamps the column when moving to a shorter line', () => {
    const buffer = TextBuffer.from('long line\nab').move(-1, 0).end().move(1, 0);
    expect([buffer.row, buffer.col]).toEqual([1, 2]);
  });
});
