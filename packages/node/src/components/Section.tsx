import {Box, Text} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';
import {SurfaceProvider} from './FocusRegion.js';

type Props = {title: string; children: ReactNode; /** Fill the remaining height (a log, a long list). */ grow?: boolean};

/**
 * One titled area of a page, drawn as a block on the panel surface with the standard padding
 * (2 columns, 1 row). The title is a header bar: bold bright text on the raised surface across the
 * block, so it outranks the group headings inside (list sections, form groups). One blank row of
 * page background separates sections. Sections never nest; nothing is placed above a section title.
 * Every section has a title: a block without a header bar is an Intro, not a section.
 */
export function Section({title, children, grow = false}: Props) {
  const theme = useTheme();
  return (
    <Box flexDirection="column" marginBottom={1} flexShrink={grow ? 1 : 0} flexGrow={grow ? 1 : 0} minHeight={0} overflow="hidden" backgroundColor={theme.tokens.panel}>
      <Box height={1} paddingX={2} backgroundColor={theme.tokens.surfaceRaised}>
        <Text bold color={theme.tokens.textBright}>{title}</Text>
      </Box>
      <Box flexDirection="column" flexGrow={grow ? 1 : 0} minHeight={0} paddingX={2} paddingY={1}>
        <SurfaceProvider surface="panel">{children}</SurfaceProvider>
      </Box>
    </Box>
  );
}
