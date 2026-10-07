import {Box, Text, useWindowSize} from 'ink';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';

export type StatusSegment = {readonly id: string; readonly text: string; readonly tone?: 'default' | 'muted' | 'accent' | 'success' | 'warning' | 'error'};

type Props = {left: readonly StatusSegment[]; right?: readonly StatusSegment[]};

/** One row on the panel surface: state segments on the left, context on the right, separated by muted dots. */
export function StatusLine({left, right = []}: Props) {
  const theme = useTheme();
  const {columns} = useWindowSize();
  // The right side keeps its width; the left side is cut to what remains (padding 2+2, one gap).
  const rightWidth = right.reduce((sum, segment, index) => sum + TextWidth.of(segment.text) + (index > 0 ? 3 : 0), 0);
  const leftWidth = Math.max(0, columns - 4 - rightWidth - (right.length > 0 ? 1 : 0));
  const tones = {
    default: theme.tokens.text,
    muted: theme.tokens.textMuted,
    accent: theme.tokens.accent,
    success: theme.tokens.success,
    warning: theme.tokens.warning,
    error: theme.tokens.error,
  };
  const render = (segments: readonly StatusSegment[], budget: number) => {
    let used = 0;
    return segments.map((segment, index) => {
      const separator = index > 0 ? 3 : 0;
      const text = TextWidth.truncate(segment.text, Math.max(0, budget - used - separator));
      used += separator + TextWidth.of(text);
      if (text === '') return null;
      return (
        <Box key={segment.id} flexDirection="row">
          {index > 0 && <Text color={theme.tokens.textMuted}> · </Text>}
          <Text color={tones[segment.tone ?? 'default']}>{text}</Text>
        </Box>
      );
    });
  };
  return (
    <Box flexDirection="row" height={1} flexShrink={0} width="100%" backgroundColor={theme.tokens.panel} paddingX={2}>
      <Box flexDirection="row" flexGrow={1} flexBasis={0} overflow="hidden">{render(left, leftWidth)}</Box>
      <Box flexDirection="row" flexShrink={0}>{render(right, rightWidth)}</Box>
    </Box>
  );
}
