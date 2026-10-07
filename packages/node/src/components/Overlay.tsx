import {Box, Text} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';

type Props = {title: string; width?: number; variant?: 'default' | 'error'; children: ReactNode};

/** A centered panel without a border: bold title at the left, "esc" muted at the right. */
export function Overlay({title, width = 60, variant = 'default', children}: Props) {
  const theme = useTheme();
  return (
    <Box flexDirection="column" width={width} backgroundColor={theme.tokens.panel} paddingX={3} paddingY={1}>
      <Box flexDirection="row" paddingX={1}>
        <Text bold color={variant === 'error' ? theme.tokens.error : theme.tokens.text}>{title}</Text>
        <Box flexGrow={1} />
        <Text color={theme.tokens.textMuted}>esc</Text>
      </Box>
      <Box height={1} />
      {children}
    </Box>
  );
}
