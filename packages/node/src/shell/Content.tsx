import {Box, useWindowSize} from 'ink';
import {Children, type ReactNode} from 'react';
import {ScreenFit} from './Fit.js';

/**
 * The row between the navigation and the bottom bars: a Sidebar and a Main side by side.
 * In a narrow terminal (ScreenFit) only the first child, the Sidebar, is kept.
 */
export function Content({children}: {children: ReactNode}) {
  const {columns, rows} = useWindowSize();
  const items = Children.toArray(children);
  const fit = ScreenFit.decide(columns, rows, items.length > 1);
  return <Box flexDirection="row" flexGrow={1} flexShrink={1} minHeight={0} overflow="hidden">{fit.main ? items : items.slice(0, 1)}</Box>;
}
