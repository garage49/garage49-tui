import {Box} from 'ink';
import {Children, type ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';

type Props = {children: ReactNode; direction?: 'row' | 'column'};

/**
 * Sub-panes side by side (or stacked): each child becomes a pane on its own surface — alternating
 * background and panel so the edge is visible without a line — with the standard padding (2 columns,
 * 1 row) and one cell between panes. Each pane takes an equal share of the space.
 */
export function Split({children, direction = 'row'}: Props) {
  const theme = useTheme();
  const items = Children.toArray(children);
  return (
    <Box flexDirection={direction} flexGrow={1} gap={1} minHeight={0}>
      {items.map((child, index) => (
        <Box
          key={index}
          flexDirection="column"
          flexGrow={1}
          flexBasis={0}
          minHeight={0}
          overflow="hidden"
          paddingX={2}
          paddingY={1}
          backgroundColor={index % 2 === 1 ? theme.tokens.panel : theme.tokens.background}
        >
          {child}
        </Box>
      ))}
    </Box>
  );
}
