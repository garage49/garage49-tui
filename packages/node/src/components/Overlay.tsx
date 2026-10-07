import {Box, Text} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';
import {SurfaceProvider} from './FocusRegion.js';

type Props = {title: string; width?: number; variant?: 'default' | 'error'; children: ReactNode};

/** A centered panel without a border: bold title at the left, "esc" muted at the right. */
export function Overlay({title, width = 60, variant = 'default', children}: Props) {
  const theme = useTheme();
  return (
    <Box flexDirection="column" width={width} backgroundColor={theme.tokens.panel} paddingX={2} paddingY={1}>
      <Box flexDirection="row">
        <Text bold color={variant === 'error' ? theme.tokens.error : theme.tokens.text}>{title}</Text>
        <Box flexGrow={1} />
        <Text color={theme.tokens.textMuted}>esc</Text>
      </Box>
      <Box height={1} />
      <SurfaceProvider surface="panel">{children}</SurfaceProvider>
    </Box>
  );
}
