import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';
import {ListLayout, type ListRow} from './ListLayout.js';

export type ListItem = {
  readonly id: string;
  readonly label: string;
  /** Right-aligned, muted: the key that triggers the item. */
  readonly shortcut?: string;
  /** Right-aligned, normal text: the item's current value (settings-style rows). */
  readonly value?: string;
  readonly section?: string;
};

type Props = {
  items: readonly ListItem[];
  selectedId?: string;
  /** Whether the list owns keyboard focus; an unfocused list shows its selection on a raised surface. */
  focused?: boolean;
  width?: number;
  /** A left click on a row. */
  onClick?: (item: ListItem) => void;
  /** The wheel moving the selection to a neighbouring row. */
  onSelect?: (item: ListItem) => void;
};

/** Rows grouped under bold colored section headings; the selected row is filled with the accent color. */
export function List({items, selectedId, focused = true, width, onClick, onSelect}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const rows = ListLayout.rows(items);
  useMouseTarget(box, {
    onPress: event => {
      const item = ListLayout.itemAt(rows, event.localY);
      if (item && event.button === 'left') onClick?.(item);
    },
    onWheel: event => {
      const index = items.findIndex(item => item.id === selectedId);
      const next = items[Math.min(items.length - 1, Math.max(0, index + (event.kind === 'wheel-up' ? -1 : 1)))];
      if (next) onSelect?.(next);
    },
  });
  return (
    <Box ref={box} flexDirection="column" width={width}>
      {rows.map(row => {
        if (row.kind === 'gap') return <Box key={row.key} height={1} />;
        if (row.kind === 'section') return <Box key={row.key}><Text bold color={theme.tokens.heading}>{row.title}</Text></Box>;
        return <ListItemRow key={row.key} row={row} selected={row.item.id === selectedId} focused={focused} />;
      })}
    </Box>
  );
}

function ListItemRow({row, selected, focused}: {row: Extract<ListRow, {kind: 'item'}>; selected: boolean; focused: boolean}) {
  const theme = useTheme();
  const {item} = row;
  const highlighted = selected && focused;
  const fill = selected ? (focused ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised) : undefined;
  const color = highlighted ? theme.tokens.selectionText : theme.tokens.text;
  const trailing = item.value ?? item.shortcut;
  const trailingColor = highlighted ? theme.tokens.selectionText : item.value ? theme.tokens.text : theme.tokens.textMuted;
  return (
    <Box backgroundColor={fill} flexDirection="row">
      <Box flexShrink={1} overflow="hidden"><Text bold={selected} color={color} wrap="truncate-end">{item.label}</Text></Box>
      <Box flexGrow={1} />
      {trailing && <Box flexShrink={0} marginLeft={2}><Text color={trailingColor}>{trailing}</Text></Box>}
    </Box>
  );
}
