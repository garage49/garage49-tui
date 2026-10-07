import {Box} from 'ink';
import type {ReactNode} from 'react';
import type {HelpEntry} from '../components/HelpOverlay.js';
import type {KeyHint} from '../components/KeyHintBar.js';
import {Screen, useOverlay} from '../components/Screen.js';
import type {StatusSegment} from '../components/StatusLine.js';
import {useKeys, type Binding} from '../input/Keys.js';
import {useAppActions, type Command} from './AppActions.js';
import {BottomBars} from './BottomBars.js';
import {FocusProvider, useFocusState} from './FocusRegistry.js';
import {StatusProvider} from './Status.js';
import {TypingProvider, useTypingState} from './Typing.js';

export type {Command} from './AppActions.js';

type Props = {
  children: ReactNode;
  /** Extra palette entries, before the built-in ones. */
  commands?: readonly Command[];
  /** Extra help lines, after the built-in keys. */
  help?: readonly HelpEntry[];
  /** Extra key hints at the bottom right, before the built-in ones. */
  hints?: readonly KeyHint[];
  /** Extra status segments after the last action. */
  status?: readonly StatusSegment[];
  /** Text at the bottom left of the key hint bar. */
  context?: string;
  /** The quit confirmation's message. */
  quitMessage?: string;
};

/**
 * The application frame: the screen, theme, mouse, overlay slot, focus cycling and the two bottom
 * bars. Put a Nav, then Content with a Sidebar and a Main inside; everything else is a default.
 */
export function App(props: Props) {
  return (
    <Screen>
      <FocusProvider>
        <StatusProvider>
          <TypingProvider>
            <Frame {...props} />
          </TypingProvider>
        </StatusProvider>
      </FocusProvider>
    </Screen>
  );
}

function Frame({children, commands = [], help = [], hints = [], status = [], context = '', quitMessage = 'Quit?'}: Props) {
  const overlay = useOverlay();
  const {registry, focusedId, setFocusedId} = useFocusState();
  const {typing} = useTypingState();
  const actions = useAppActions(commands, help, quitMessage);
  const move = (delta: number, wrap = false) => setFocusedId(registry.neighbour(focusedId, delta, wrap)?.id);
  const descends = registry.find(focusedId)?.options.descendOnEnter === true;
  const letters: readonly Binding[] = typing ? [] : [
    {keys: ['?'], run: actions.openHelp},
    {keys: ['q'], run: actions.confirmQuit},
    {keys: ['t'], run: actions.toggleTheme},
    {keys: ['m'], run: actions.toggleMouse},
  ];
  useKeys([
    {keys: ['ctrl+p'], run: actions.openPalette},
    {keys: ['tab'], run: () => move(1, true)},
    {keys: ['shift+tab'], run: () => move(-1, true)},
    {keys: ['esc'], run: () => move(-1)},
    ...(descends ? [{keys: ['enter'], run: () => move(1)}] : []),
    ...letters,
  ], {isActive: !overlay.isOpen});
  return (
    <>
      <Box flexDirection="column" flexGrow={1}>{children}</Box>
      <BottomBars actions={actions} hints={hints} status={status} context={context} onNext={() => move(1, true)} />
    </>
  );
}
