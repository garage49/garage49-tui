import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {TreeLayout, TreeView, useKeys} from '@garage49/garage49-tui-ink';
import type {TreeNode, TreeRow} from '@garage49/garage49-tui-ink';

const nodes: readonly TreeNode[] = [
  {id: 'src', label: 'src', children: [
    {id: 'src/components', label: 'components', children: [
      {id: 'src/components/List.tsx', label: 'List.tsx'},
      {id: 'src/components/Table.tsx', label: 'Table.tsx'},
      {id: 'src/components/TreeView.tsx', label: 'TreeView.tsx'},
    ]},
    {id: 'src/theme', label: 'theme', children: [
      {id: 'src/theme/Theme.ts', label: 'Theme.ts'},
      {id: 'src/theme/ThemeContext.tsx', label: 'ThemeContext.tsx'},
    ]},
    {id: 'src/main.tsx', label: 'main.tsx'},
  ]},
  {id: 'docs', label: '문서', children: [
    {id: 'docs/HANDOFF.md', label: 'HANDOFF.md'},
    {id: 'docs/DESIGN.md', label: 'DESIGN.md'},
  ]},
  {id: 'package.json', label: 'package.json'},
];

export function TreePage({focused, report}: PageProps) {
  const [expanded, setExpanded] = useState<ReadonlySet<string>>(new Set(['src', 'src/components']));
  const [selectedId, setSelectedId] = useState<string | undefined>('src');
  const rows = TreeLayout.rows(nodes, expanded);
  const index = rows.findIndex(row => row.node.id === selectedId);
  const current = rows[index];

  const toggle = (row: TreeRow, open?: boolean) => {
    if (!row.node.children) return report(`open: ${row.node.label}`);
    const next = new Set(expanded);
    const willOpen = open ?? !next.has(row.node.id);
    if (willOpen) next.add(row.node.id); else next.delete(row.node.id);
    setExpanded(next);
  };
  const move = (delta: number) => {
    const next = rows[Math.min(rows.length - 1, Math.max(0, index + delta))];
    if (next) setSelectedId(next.node.id);
  };

  const expand = () => {
    if (!current) return;
    if (current.node.children && !current.expanded) toggle(current, true); else move(1);
  };
  const collapse = () => {
    if (!current) return;
    if (current.expanded) toggle(current, false); else if (current.parentId) setSelectedId(current.parentId);
  };
  useKeys([
    {keys: ['up', 'k'], run: () => move(-1)},
    {keys: ['down', 'j'], run: () => move(1)},
    {keys: ['right', 'l'], run: expand},
    {keys: ['left', 'h'], run: collapse},
    {keys: ['enter', 'space'], run: () => current && toggle(current)},
  ], {isActive: focused});

  return (
    <Box flexDirection="column" width={50}>
      <TreeView
        nodes={nodes}
        expanded={expanded}
        selectedId={selectedId}
        focused={focused}
        onClick={row => { setSelectedId(row.node.id); toggle(row); }}
        onSelect={row => setSelectedId(row.node.id)}
      />
    </Box>
  );
}
