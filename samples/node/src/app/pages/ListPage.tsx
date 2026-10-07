import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {List, useKeys, Selection} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';

const items: readonly ListItem[] = [
  {id: 'r1', label: 'garage49-tui', value: '2 min ago', section: 'Recent projects'},
  {id: 'r2', label: '한글 프로젝트 aterm', value: 'yesterday', section: 'Recent projects'},
  {id: 'r3', label: 'skills', value: '3 days ago', section: 'Recent projects'},
  {id: 'a1', label: 'Open a project', shortcut: 'ctrl+o', section: 'Actions'},
  {id: 'a2', label: 'Command palette', shortcut: 'ctrl+p', section: 'Actions'},
  {id: 'a3', label: 'A very long label that does not fit in the list width and gets truncated', section: 'Actions'},
  {id: 's1', label: 'Theme', value: 'opencode', section: 'Settings'},
  {id: 's2', label: 'Mouse', value: 'on', section: 'Settings'},
];

export function ListPage({focused, report}: PageProps) {
  const [selectedId, setSelectedId] = useState<string | undefined>(items[0]?.id);
  const activate = (item: ListItem) => report(`activated: ${item.label}`);
  useKeys([
    {keys: ['up', 'k'], run: () => setSelectedId(Selection.move(items, selectedId, -1))},
    {keys: ['down', 'j'], run: () => setSelectedId(Selection.move(items, selectedId, 1))},
    {keys: ['enter'], run: () => { const item = items.find(candidate => candidate.id === selectedId); if (item) activate(item); }},
  ], {isActive: focused});
  return (
    <Box flexDirection="column" width={56}>
      <List
        items={items}
        selectedId={selectedId}
        focused={focused}
        onClick={item => { setSelectedId(item.id); activate(item); }}
        onSelect={item => setSelectedId(item.id)}
      />
    </Box>
  );
}
