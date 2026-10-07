import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';
import {TreeClick} from './TreeClick.js';

export type TreeNode = {
  readonly id: string;
  readonly label: string;
  readonly children?: readonly TreeNode[];
};

/** One visible row of a flattened tree. */
export type TreeRow = {readonly node: TreeNode; readonly depth: number; readonly expanded: boolean; readonly parentId?: string};

/** Flattens a tree into the rows that are visible given the expanded set. */
export class TreeLayout {
  /** Whether `id` is `ancestorId` itself or lies anywhere under it. */
  static contains(nodes: readonly TreeNode[], ancestorId: string, id: string): boolean {
    const find = (list: readonly TreeNode[]): TreeNode | undefined => {
      for (const node of list) {
        if (node.id === ancestorId) return node;
        const inner = node.children ? find(node.children) : undefined;
        if (inner) return inner;
      }
      return undefined;
    };
    const has = (node: TreeNode): boolean => node.id === id || (node.children ?? []).some(has);
    const ancestor = find(nodes);
    return ancestor !== undefined && has(ancestor);
  }

  /**
   * The cursor after `branchId` collapses: a cursor inside the branch would vanish, so it moves to
   * the branch; any other cursor stays.
   */
  static cursorAfterCollapse(nodes: readonly TreeNode[], branchId: string, cursorId: string | undefined): string | undefined {
    return cursorId !== undefined && TreeLayout.contains(nodes, branchId, cursorId) ? branchId : cursorId;
  }

  static rows(nodes: readonly TreeNode[], expanded: ReadonlySet<string>, depth = 0, parentId?: string): TreeRow[] {
    return nodes.flatMap(node => {
      const open = expanded.has(node.id);
      const row: TreeRow = {node, depth, expanded: open, parentId};
      return open && node.children ? [row, ...TreeLayout.rows(node.children, expanded, depth + 1, node.id)] : [row];
    });
  }
}

type Props = {
  nodes: readonly TreeNode[];
  expanded: ReadonlySet<string>;
  selectedId?: string;
  focused?: boolean;
  /** The cursor moves to a row (click on an unselected row, or the wheel). */
  onSelect?: (row: TreeRow) => void;
  /** A branch opens or closes (click on the ▸/▾ marker, or on the row that is already selected). */
  onToggle?: (row: TreeRow) => void;
};

/**
 * Indented rows with ▸/▾ markers for branches; the selected row is filled with the accent color.
 * Mouse, by the file-manager convention: a click selects a row, a click on its marker or on the
 * already selected row toggles it; the wheel moves the cursor.
 */
export function TreeView({nodes, expanded, selectedId, focused = true, onSelect, onToggle}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const rows = TreeLayout.rows(nodes, expanded);
  useMouseTarget(box, {
    onPress: event => {
      const row = rows[event.localY];
      if (!row || event.button !== 'left') return;
      const action = TreeClick.decide(TreeClick.isMarker(event.localX, row.depth), row.node.id === selectedId);
      if (action === 'toggle') onToggle?.(row); else onSelect?.(row);
    },
    onWheel: event => {
      const index = rows.findIndex(row => row.node.id === selectedId);
      const next = rows[Math.min(rows.length - 1, Math.max(0, index + (event.kind === 'wheel-up' ? -1 : 1)))];
      if (next) onSelect?.(next);
    },
  });
  return (
    <Box ref={box} flexDirection="column">
      {rows.map(row => {
        const selected = row.node.id === selectedId;
        const highlighted = selected && focused;
        const fill = selected ? (focused ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised) : undefined;
        const color = highlighted ? theme.tokens.selectionText : theme.tokens.text;
        const marker = row.node.children ? `${row.expanded ? Glyphs.expanded : Glyphs.collapsed} ` : '  ';
        return (
          <Box key={row.node.id} backgroundColor={fill} paddingX={1} flexDirection="row">
            <Text color={highlighted ? theme.tokens.selectionText : theme.tokens.textMuted}>{'  '.repeat(row.depth)}{marker}</Text>
            <Text bold={selected} color={color} wrap="truncate-end">{row.node.label}</Text>
          </Box>
        );
      })}
    </Box>
  );
}
