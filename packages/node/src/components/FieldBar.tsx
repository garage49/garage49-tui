import {Box, Text} from 'ink';
import {useTheme} from '../theme/ThemeContext.js';
import type {RegionSurface} from './FocusRegion.js';

type Props = {focused: boolean; surface?: RegionSurface; /** Height of the field; the bar covers every row. */ rows?: number};

/**
 * The first column of a form field: blank normally, a blue ┃ down every row while the field is being edited.
 * Same rule as FocusRegion, at field scale: the bar is painted over the field's own first column.
 */
export function FieldBar({focused, surface = 'surface', rows = 1}: Props) {
  const theme = useTheme();
  return (
    <Box width={1} height={rows} backgroundColor={theme.tokens[surface]}>
      <Text color={theme.tokens.accentSecondary}>{(focused ? '┃\n' : ' \n').repeat(rows).trimEnd()}</Text>
    </Box>
  );
}
