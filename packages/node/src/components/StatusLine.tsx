import {Box, Text} from 'ink';
import {useTheme} from '../theme/ThemeContext.js';

export type StatusSegment = {readonly id: string; readonly text: string; readonly tone?: 'default' | 'muted' | 'accent' | 'success' | 'warning' | 'error'};

type Props = {left: readonly StatusSegment[]; right?: readonly StatusSegment[]};

/** One row on the panel surface: state segments on the left, context on the right, separated by muted dots. */
export function StatusLine({left, right = []}: Props) {
  const theme = useTheme();
  const tones = {
    default: theme.tokens.text,
    muted: theme.tokens.textMuted,
    accent: theme.tokens.accent,
    success: theme.tokens.success,
    warning: theme.tokens.warning,
    error: theme.tokens.error,
  };
  const render = (segments: readonly StatusSegment[]) =>
    segments.map((segment, index) => (
      <Box key={segment.id} flexDirection="row">
        {index > 0 && <Text color={theme.tokens.textMuted}> · </Text>}
        <Text color={tones[segment.tone ?? 'default']}>{segment.text}</Text>
      </Box>
    ));
  return (
    <Box flexDirection="row" height={1} flexShrink={0} width="100%" backgroundColor={theme.tokens.panel} paddingX={2}>
      <Box flexDirection="row" flexGrow={1} flexBasis={0} overflow="hidden">{render(left)}</Box>
      <Box flexDirection="row" flexShrink={0}>{render(right)}</Box>
    </Box>
  );
}
