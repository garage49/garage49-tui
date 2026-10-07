import type {Key} from 'ink';
import {describe, expect, it} from 'vitest';
import {KeyChord} from '../src/input/KeyChord.js';

/** Every flag off; cast because Ink keeps adding flags (capsLock, numLock, …) that the chord ignores. */
const none = {upArrow: false, downArrow: false, leftArrow: false, rightArrow: false, pageDown: false, pageUp: false, home: false, end: false, return: false, escape: false, ctrl: false, shift: false, tab: false, backspace: false, delete: false, meta: false, super: false, hyper: false} as Key;
const key = (flags: Partial<Key>): Key => ({...none, ...flags});

describe('KeyChord', () => {
  it('names arrows, enter, esc and space', () => {
    expect(KeyChord.of('', key({upArrow: true}))).toBe('up');
    expect(KeyChord.of('\r', key({return: true}))).toBe('enter');
    expect(KeyChord.of('', key({escape: true}))).toBe('esc');
    expect(KeyChord.of(' ', none)).toBe('space');
  });

  it('prefixes modifiers and keeps printable characters as themselves', () => {
    expect(KeyChord.of('p', key({ctrl: true}))).toBe('ctrl+p');
    expect(KeyChord.of('f', key({meta: true}))).toBe('alt+f');
    expect(KeyChord.of('', key({tab: true, shift: true}))).toBe('shift+tab');
    expect(KeyChord.of('?', none)).toBe('?');
    expect(KeyChord.of('한', none)).toBe('한');
  });

  it('treats plain characters as text but not control chords or named keys', () => {
    expect(KeyChord.text('q', none)).toBe('q');
    expect(KeyChord.text('p', key({ctrl: true}))).toBeUndefined();
    expect(KeyChord.text('', key({upArrow: true}))).toBeUndefined();
  });
});
