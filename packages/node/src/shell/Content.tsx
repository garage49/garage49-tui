import {Box} from 'ink';
import type {ReactNode} from 'react';

/** The row between the navigation and the bottom bars: a Sidebar and a Main side by side. */
export function Content({children}: {children: ReactNode}) {
  return <Box flexDirection="row" flexGrow={1}>{children}</Box>;
}
