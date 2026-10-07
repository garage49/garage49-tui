import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';

export type Column<Row> = {
  readonly id: string;
  readonly title: string;
  readonly width: number;
  readonly align?: 'left' | 'right';
  readonly cell: (row: Row) => string;
};

type Props<Row> = {
  columns: readonly Column<Row>[];
  rows: readonly Row[];
  rowId: (row: Row) => string;
  selectedId?: string;
  focused?: boolean;
  onClick?: (row: Row) => void;
  onSelect?: (row: Row) => void;
};

/** Fixed-width columns with a muted bold header; the selected row is filled with the accent color. Click selects. */
export function Table<Row>({columns, rows, rowId, selectedId, focused = true, onClick, onSelect}: Props<Row>) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {
    onPress: event => {
      const row = rows[event.localY - 1];
      if (row && event.button === 'left') onClick?.(row);
    },
    onWheel: event => {
      const index = rows.findIndex(row => rowId(row) === selectedId);
      const next = rows[Math.min(rows.length - 1, Math.max(0, index + (event.kind === 'wheel-up' ? -1 : 1)))];
      if (next) onSelect?.(next);
    },
  });
  const line = (cells: readonly string[], color: string, bold: boolean) =>
    columns.map((column, index) => (
      <Box key={column.id} width={column.width} flexShrink={0} marginRight={2} justifyContent={column.align === 'right' ? 'flex-end' : 'flex-start'}>
        <Text bold={bold} color={color} wrap="truncate-end">{cells[index]}</Text>
      </Box>
    ));
  return (
    <Box ref={box} flexDirection="column">
      <Box flexDirection="row">{line(columns.map(column => column.title), theme.tokens.textMuted, true)}</Box>
      {rows.map(row => {
        const id = rowId(row);
        const selected = id === selectedId;
        const highlighted = selected && focused;
        const fill = selected ? (focused ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised) : undefined;
        return (
          <Box key={id} flexDirection="row" backgroundColor={fill}>
            {line(columns.map(column => column.cell(row)), highlighted ? theme.tokens.selectionText : theme.tokens.text, selected)}
          </Box>
        );
      })}
    </Box>
  );
}
