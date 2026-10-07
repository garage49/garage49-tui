import {Box, type BoxProps, type DOMElement} from 'ink';
import type {ReactNode, Ref} from 'react';
import {useTheme} from '../theme/ThemeContext.js';

type Props = BoxProps & {children?: ReactNode; raised?: boolean; ref?: Ref<DOMElement>};

/** A region separated from its neighbours by background color only — no border lines. */
export function Panel({children, raised = false, ...box}: Props) {
  const theme = useTheme();
  return (
    <Box flexDirection="column" backgroundColor={raised ? theme.tokens.surface : theme.tokens.panel} paddingX={2} paddingY={1} {...box}>
      {children}
    </Box>
  );
}
