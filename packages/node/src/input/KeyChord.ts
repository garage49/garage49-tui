import type {Key} from 'ink';

/**
 * The name of a key press as bindings spell it: 'up', 'enter', 'esc', 'space', 'shift+tab',
 * 'ctrl+p', 'alt+f', or the printable character itself ('q', '?', 'k').
 */
export class KeyChord {
  private static readonly names: readonly (readonly [keyof Key, string])[] = [
    ['upArrow', 'up'], ['downArrow', 'down'], ['leftArrow', 'left'], ['rightArrow', 'right'],
    ['pageUp', 'pageup'], ['pageDown', 'pagedown'], ['home', 'home'], ['end', 'end'],
    ['return', 'enter'], ['escape', 'esc'], ['tab', 'tab'], ['backspace', 'backspace'], ['delete', 'delete'],
  ];

  static of(input: string, key: Key): string | undefined {
    const named = KeyChord.names.find(([flag]) => key[flag])?.[1];
    const base = named ?? (input === ' ' ? 'space' : input.length === 1 ? input : undefined);
    if (base === undefined) return undefined;
    const prefix = (key.ctrl ? 'ctrl+' : '') + (key.meta ? 'alt+' : '') + (key.shift && named ? 'shift+' : '');
    return prefix + base;
  }

  /** Printable text that a text field would insert: one or more characters without ctrl/alt and not a named key. */
  static text(input: string, key: Key): string | undefined {
    if (!input || key.ctrl || key.meta) return undefined;
    if (KeyChord.names.some(([flag]) => key[flag])) return undefined;
    return input;
  }
}
