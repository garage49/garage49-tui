import {Box, Text} from 'ink';
import {useTheme} from '../theme/ThemeContext.js';

/** Where the thumb sits on a track of `rows` cells, for `visible` of `total` lines starting at `offset`. */
export class ScrollThumb {
  static place(rows: number, total: number, visible: number, offset: number): {start: number; size: number} {
    if (total <= visible || rows <= 0) return {start: 0, size: rows};
    const size = Math.max(1, Math.round((rows * visible) / total));
    const start = Math.round(((rows - size) * offset) / (total - visible));
    return {start, size};
  }
}

type Props = {rows: number; total: number; visible: number; offset: number};

/** A one-column scrollbar: a faint track with a thumb whose size and position show how much is above and below. Hidden when everything fits. */
export function Scrollbar({rows, total, visible, offset}: Props) {
  const theme = useTheme();
  if (total <= visible) return <Box width={1} height={rows} />;
  const {start, size} = ScrollThumb.place(rows, total, visible, offset);
  return (
    <Box width={1} height={rows} flexDirection="column">
      {Array.from({length: rows}, (_, row) => {
        const thumb = row >= start && row < start + size;
        return <Text key={row} color={thumb ? theme.tokens.textMuted : theme.tokens.surfaceRaised}>{thumb ? '┃' : '│'}</Text>;
      })}
    </Box>
  );
}
