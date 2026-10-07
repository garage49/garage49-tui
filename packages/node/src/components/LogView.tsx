import {Box, Text, useBoxMetrics, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';
import {Scrollbar} from './Scrollbar.js';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export type LogEntry = {readonly time: string; readonly level: LogLevel; readonly message: string};

type Props = {
  entries: readonly LogEntry[];
  /** Rows scrolled up from the bottom; 0 follows the newest entry. */
  offset: number;
  onScroll?: (offset: number) => void;
};

/** A scrolling log: fixed time column, level colored by severity, newest at the bottom. The wheel scrolls; a scrollbar shows the position. */
export function LogView({entries, offset, onScroll}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const {clientHeight} = useBoxMetrics(box);
  const height = Math.max(1, clientHeight);
  const maxOffset = Math.max(0, entries.length - height);
  const clamped = Math.min(offset, maxOffset);
  const end = entries.length - clamped;
  const visible = entries.slice(Math.max(0, end - height), end);
  useMouseTarget(box, {
    onWheel: event => onScroll?.(Math.min(maxOffset, Math.max(0, clamped + (event.kind === 'wheel-up' ? 3 : -3)))),
  });
  const levelColor: Record<LogLevel, string> = {
    debug: theme.tokens.textMuted,
    info: theme.tokens.accentSecondary,
    warn: theme.tokens.warning,
    error: theme.tokens.error,
  };
  return (
    <Box ref={box} flexDirection="row" flexGrow={1} overflow="hidden">
      <Box flexDirection="column" flexGrow={1} overflow="hidden">
        {visible.map((entry, index) => (
          <Box key={`${end - visible.length + index}`} flexDirection="row">
            <Box width={9} flexShrink={0}><Text color={theme.tokens.textMuted}>{entry.time}</Text></Box>
            <Box width={6} flexShrink={0}><Text bold color={levelColor[entry.level]}>{entry.level.toUpperCase()}</Text></Box>
            <Text color={entry.level === 'error' ? theme.tokens.error : theme.tokens.text} wrap="truncate-end">{entry.message}</Text>
          </Box>
        ))}
      </Box>
      <Scrollbar rows={height} total={entries.length} visible={height} offset={Math.max(0, end - height)} />
    </Box>
  );
}
