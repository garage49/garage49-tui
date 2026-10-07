import {render, type Instance} from 'ink';
import type {ReactNode} from 'react';
import {ThemeRoot} from '../theme/ThemeContext.js';
import {Theme, opencodeTheme, systemTheme} from '../theme/Theme.js';

/** Picks the theme for this terminal: OpenCode's truecolor theme when available, the system theme otherwise. */
export class TerminalTheme {
  static detect(env: NodeJS.ProcessEnv = process.env): Theme {
    const truecolor = env.COLORTERM === 'truecolor' || env.COLORTERM === '24bit';
    return env.G49_THEME === 'system' || !truecolor ? systemTheme : opencodeTheme;
  }
}

/** Mounts an App on the alternate screen with the detected theme; ctrl+c exits and restores the terminal. */
export function run(app: ReactNode, options: {theme?: Theme} = {}): Instance {
  return render(<ThemeRoot initial={options.theme ?? TerminalTheme.detect()}>{app}</ThemeRoot>, {alternateScreen: true, exitOnCtrlC: true});
}
