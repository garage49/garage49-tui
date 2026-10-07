import {Box, Text} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';
import {SurfaceProvider} from './FocusRegion.js';

type Props = {title: string; children?: ReactNode};

/**
 * A page's introduction: a block on the panel surface with no header bar, at the top of the page
 * before its sections, holding one bright title line and a short muted explanation. It is not a
 * section (nothing is listed or edited in it), so it has no header bar; at most one per page.
 */
export function Intro({title, children}: Props) {
  const theme = useTheme();
  return (
    <Box flexDirection="column" marginBottom={1} flexShrink={0} backgroundColor={theme.tokens.panel} paddingX={2} paddingY={1}>
      <Text color={theme.tokens.textBright}>{title}</Text>
      <SurfaceProvider surface="panel">{children}</SurfaceProvider>
    </Box>
  );
}
