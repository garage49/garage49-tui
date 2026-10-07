import {useInput, type Key} from 'ink';
import {KeyChord} from './KeyChord.js';
import {MouseParser} from './MouseParser.js';

/** One key binding: any of `keys` runs `run`. The names are KeyChord names. */
export type Binding = {readonly keys: readonly string[]; readonly run: () => void};

export type KeysOptions = {
  isActive?: boolean;
  /** Receives printable text that no binding claimed (text fields). */
  onText?: (text: string) => void;
  /** Receives every key that no binding claimed, as (chord, input, key). */
  onOther?: (chord: string | undefined, input: string, key: Key) => void;
};

/**
 * Keyboard input for components, as a table of bindings. Wraps Ink's useInput, drops mouse reports,
 * and runs the first binding whose keys match. Components never call useInput directly.
 */
export function useKeys(bindings: readonly Binding[], options: KeysOptions = {}): void {
  useInput((input, key) => {
    if (MouseParser.isReport(input)) return;
    const chord = KeyChord.of(input, key);
    const binding = chord === undefined ? undefined : bindings.find(candidate => candidate.keys.includes(chord));
    if (binding) return binding.run();
    const text = KeyChord.text(input, key);
    if (text !== undefined && options.onText) return options.onText(text);
    options.onOther?.(chord, input, key);
  }, {isActive: options.isActive});
}
