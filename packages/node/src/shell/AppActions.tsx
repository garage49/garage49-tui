import {useApp} from 'ink';
import {ConfirmDialog} from '../components/ConfirmDialog.js';
import {HelpOverlay, type HelpEntry} from '../components/HelpOverlay.js';
import type {ListItem} from '../components/List.js';
import {Palette} from '../components/Palette.js';
import {useOverlay} from '../components/Screen.js';
import {useMouseSwitch} from '../input/Mouse.js';
import {opencodeTheme, systemTheme} from '../theme/Theme.js';
import {useTheme, useThemeSwitch} from '../theme/ThemeContext.js';
import {useStatus} from './Status.js';

export type Command = ListItem & {readonly run: () => void};

export type AppActions = {
  openPalette: () => void;
  openHelp: () => void;
  confirmQuit: () => void;
  toggleTheme: () => void;
  toggleMouse: () => void;
};

const builtinHelp: readonly HelpEntry[] = [
  {keys: 'tab / shift+tab', action: 'cycle focus through the regions, top to bottom'},
  {keys: 'enter / esc', action: 'go down into the next region / back up (arrows stay inside a region)'},
  {keys: '↑↓ / jk', action: 'move in the focused list'},
  {keys: 'ctrl+p', action: 'command palette'},
  {keys: 't', action: 'toggle theme'},
  {keys: 'm', action: 'toggle the mouse (off: the terminal selects text again; on: use shift+drag)'},
  {keys: '?  q', action: 'help · quit'},
  {keys: 'mouse', action: 'click a region to focus it; click tabs, rows, buttons, hints; wheel scrolls'},
];

/** The App's built-in actions: palette, help, quit confirmation, theme and mouse toggles. */
export function useAppActions(commands: readonly Command[], help: readonly HelpEntry[], quitMessage: string): AppActions {
  const {exit} = useApp();
  const theme = useTheme();
  const switchTheme = useThemeSwitch();
  const mouse = useMouseSwitch();
  const overlay = useOverlay();
  const {report} = useStatus();

  const confirmQuit = () => overlay.show(<ConfirmDialog title="Quit" message={quitMessage} confirmLabel="Quit" onConfirm={exit} onCancel={overlay.hide} />);
  const openHelp = () => overlay.show(<HelpOverlay entries={[...builtinHelp, ...help]} onClose={overlay.hide} />);
  const toggleTheme = () => {
    const next = theme.name.startsWith('system') ? opencodeTheme : systemTheme;
    switchTheme(next);
    report(`theme: ${next.name}`);
  };
  const toggleMouse = () => {
    mouse.setEnabled(!mouse.enabled);
    report(`mouse: ${mouse.enabled ? 'off' : 'on'}`);
  };
  const all: readonly Command[] = [
    ...commands,
    {id: 'app.theme', label: 'Toggle theme', shortcut: 't', section: 'Application', run: toggleTheme},
    {id: 'app.mouse', label: 'Toggle mouse', shortcut: 'm', section: 'Application', run: toggleMouse},
    {id: 'app.help', label: 'Keyboard shortcuts', shortcut: '?', section: 'Application', run: openHelp},
    {id: 'app.quit', label: 'Quit', shortcut: 'q', section: 'Application', run: confirmQuit},
  ];
  const openPalette = () =>
    overlay.show(<Palette title="Commands" items={all} onClose={overlay.hide} onPick={item => { overlay.hide(); all.find(c => c.id === item.id)?.run(); }} />);
  return {openPalette, openHelp, confirmQuit, toggleTheme, toggleMouse};
}
