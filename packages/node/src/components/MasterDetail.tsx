import {Box} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';
import {SurfaceProvider} from './FocusRegion.js';
import {List, type ListItem} from './List.js';
import {ListLayout} from './ListLayout.js';

type Props = {
  items: readonly ListItem[];
  selectedId?: string;
  focused?: boolean;
  /** A left click on a row. */
  onClick?: (item: ListItem) => void;
  /** The wheel moving the selection to a neighbouring row. */
  onSelect?: (item: ListItem) => void;
  /** The detail of the selected row. */
  children: ReactNode;
};

/**
 * A list and the detail of its selected row as one control inside a section: the list takes the
 * left third, the detail the rest on the surface token, and the selected row's fill continues across
 * the one-cell gap into the detail, so the two read as connected rather than as two sections.
 */
export function MasterDetail({items, selectedId, focused = true, onClick, onSelect, children}: Props) {
  const theme = useTheme();
  const row = ListLayout.rows(items).findIndex(r => r.kind === 'item' && r.item.id === selectedId);
  const fill = focused ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised;
  return (
    <Box flexDirection="row" flexGrow={1} minHeight={0}>
      <Box flexDirection="column" flexGrow={1} flexBasis={0} minHeight={0}>
        <List items={items} selectedId={selectedId} focused={focused} onClick={onClick} onSelect={onSelect} />
      </Box>
      <Box flexDirection="column" width={1} flexShrink={0} paddingTop={Math.max(0, row)}>
        {row >= 0 && <Box height={1} backgroundColor={fill} />}
      </Box>
      <Box flexDirection="column" flexGrow={2} flexBasis={0} minHeight={0} backgroundColor={theme.tokens.surface} paddingX={2} paddingY={1}>
        <SurfaceProvider surface="surface">{children}</SurfaceProvider>
      </Box>
    </Box>
  );
}
