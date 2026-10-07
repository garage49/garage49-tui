import {Box, Text, useBoxMetrics, type DOMElement} from 'ink';
import {createContext, useContext, useRef, type ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';

export type RegionSurface = 'background' | 'panel' | 'surface' | 'heading';

const SurfaceContext = createContext<RegionSurface>('background');

/** Tells the fields inside which color their container draws, so a bar-only field paints its bar column on it. */
export function SurfaceProvider({surface, children}: {surface: RegionSurface; children: ReactNode}) {
  return <SurfaceContext.Provider value={surface}>{children}</SurfaceContext.Provider>;
}

/** The color of the nearest container: a Section's panel, a MasterDetail detail's surface, an Overlay's panel, else the background. */
export function useSurface(): RegionSurface {
  return useContext(SurfaceContext);
}

type Props = {
  focused: boolean;
  /** Take the remaining width of the parent row. */
  grow?: boolean;
  /** What the region draws in its first column; the bar is painted on that color. One value, or one per row (the last repeats). */
  surface?: RegionSurface | readonly RegionSurface[];
  /** How many rows the bar covers from the top; the whole region by default. A header marks only its title row. */
  rows?: number;
  children: ReactNode;
};

/**
 * A focusable region. When focused, a column of ┃ in accentSecondary (blue) is painted over the
 * region's first column; otherwise nothing is drawn, so the region looks exactly as it did.
 * The region must keep its first column free (padding-left 1). Ink does not keep the cell's
 * background when a character is painted over it, so the bar repaints the region's surface.
 */
export function FocusRegion({focused, grow = false, surface = 'background', rows, children}: Props) {
  const theme = useTheme();
  const content = useRef<DOMElement>(null);
  const {height} = useBoxMetrics(content);
  const barHeight = Math.max(1, Math.min(height, rows ?? height));
  return (
    <Box flexDirection="row" flexGrow={grow ? 1 : 0} flexShrink={grow ? 1 : 0}>
      <Box ref={content} flexDirection="column" flexGrow={1}>{children}</Box>
      {focused && (
        <Box position="absolute" top={0} left={0} width={1} height={barHeight} overflow="hidden" flexDirection="column">
          {Array.from({length: barHeight}, (_, row) => {
            const surfaces = typeof surface === 'string' ? [surface] : surface;
            const name = surfaces[Math.min(row, surfaces.length - 1)]!;
            return <Box key={row} height={1} backgroundColor={theme.tokens[name]}><Text color={theme.tokens.accentSecondary}>┃</Text></Box>;
          })}
        </Box>
      )}
    </Box>
  );
}
