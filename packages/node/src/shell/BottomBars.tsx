import {Box, useWindowSize} from 'ink';
import {useEffect, useState} from 'react';
import {KeyHintBar, type KeyHint} from '../components/KeyHintBar.js';
import {StatusLine, type StatusSegment} from '../components/StatusLine.js';
import {useMouseSwitch} from '../input/Mouse.js';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';
import type {AppActions} from './AppActions.js';
import {useStatus} from './Status.js';

type Props = {
  actions: AppActions;
  hints: readonly KeyHint[];
  status: readonly StatusSegment[];
  context: string;
  /** Tab's hint: moves the focus to the next region. */
  onNext: () => void;
};

const builtinHints: readonly KeyHint[] = [{key: 'tab', label: 'focus'}, {key: 'ctrl+p', label: 'commands'}, {key: '?', label: 'help'}, {key: 'q', label: 'quit'}];

/** The status line (last action, app segments, mouse, theme, size, clock) and the key hint bar. */
export function BottomBars({actions, hints, status, context, onNext}: Props) {
  const theme = useTheme();
  const mouse = useMouseSwitch();
  const {columns, rows} = useWindowSize();
  const {lastAction} = useStatus();
  const [clock, setClock] = useState(() => new Date().toTimeString().slice(0, 5));
  useEffect(() => {
    const timer = setInterval(() => setClock(new Date().toTimeString().slice(0, 5)), 10_000);
    return () => clearInterval(timer);
  }, []);
  const onHint = (hint: KeyHint) => {
    const run: Record<string, () => void> = {tab: onNext, 'ctrl+p': actions.openPalette, '?': actions.openHelp, q: actions.confirmQuit};
    run[hint.key]?.();
  };
  return (
    <>
      <StatusLine
        left={[{id: 'app.state', text: `${Glyphs.on} ${lastAction}`, tone: 'success'}, ...status]}
        right={[
          {id: 'app.mouse', text: mouse.enabled ? 'mouse' : 'no mouse', tone: mouse.enabled ? 'muted' : 'warning'},
          {id: 'app.theme', text: theme.name.replace('-dimmed', ''), tone: 'muted'},
          {id: 'app.size', text: `${columns}×${rows}`, tone: 'muted'},
          {id: 'app.clock', text: clock},
        ]}
      />
      <Box paddingX={2}>
        <KeyHintBar left={context} hints={[...hints, ...builtinHints]} onPress={onHint} />
      </Box>
    </>
  );
}
