import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {ConfirmDialog, List, MessageDialog, Palette, useOverlay, useKeys, Selection} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';

const sampleCommands: readonly ListItem[] = [
  {id: 'new', label: 'New project', shortcut: 'ctrl+n', section: 'File'},
  {id: 'open', label: 'Open…', shortcut: 'ctrl+o', section: 'File'},
  {id: 'rename', label: 'Rename', section: 'Edit'},
  {id: 'delete', label: 'Delete', section: 'Edit'},
  {id: 'shortcuts', label: 'Keyboard shortcuts', shortcut: '?', section: 'Help'},
];

const items: readonly ListItem[] = [
  {id: 'confirm', label: 'Confirm dialog', section: 'Dialogs'},
  {id: 'danger', label: 'Confirm dialog (destructive)', section: 'Dialogs'},
  {id: 'error', label: 'Error dialog', section: 'Dialogs'},
  {id: 'message', label: 'Message dialog', section: 'Dialogs'},
  {id: 'palette', label: 'Command palette', shortcut: 'ctrl+p', section: 'Pop-ups'},
];

export function DialogsPage({focused, report}: PageProps) {
  const overlay = useOverlay();
  const [selectedId, setSelectedId] = useState<string | undefined>(items[0]?.id);
  const open = (item: ListItem) => {
    const done = (text: string) => { overlay.hide(); report(text); };
    switch (item.id) {
      case 'confirm':
        return overlay.show(<ConfirmDialog title="Save changes" message="Save the current project before closing it?" confirmLabel="Save" onConfirm={() => done('saved')} onCancel={() => done('cancelled')} />);
      case 'danger':
        return overlay.show(<ConfirmDialog danger title="Delete project" message="Delete 한글 프로젝트 aterm? This cannot be undone." confirmLabel="Delete" onConfirm={() => done('deleted')} onCancel={() => done('cancelled')} />);
      case 'error':
        return overlay.show(<MessageDialog variant="error" title="Connection failed" message="Could not reach upstream (timeout after 30s). Check the network and try again." onClose={() => done('error dismissed')} />);
      case 'message':
        return overlay.show(<MessageDialog title="About garage49" message="TUI design system sample · Ink 8 · React 19" onClose={() => done('closed')} />);
      case 'palette':
        return overlay.show(<Palette title="Commands" items={sampleCommands} onClose={overlay.hide} onPick={picked => done(`palette: ${picked.label}`)} />);
    }
  };
  useKeys([
    {keys: ['up', 'k'], run: () => setSelectedId(Selection.move(items, selectedId, -1))},
    {keys: ['down', 'j'], run: () => setSelectedId(Selection.move(items, selectedId, 1))},
    {keys: ['enter'], run: () => { const item = items.find(candidate => candidate.id === selectedId); if (item) open(item); }},
  ], {isActive: focused});
  return (
    <Box flexDirection="column" width={50}>
      <List items={items} selectedId={selectedId} focused={focused} onClick={item => { setSelectedId(item.id); open(item); }} onSelect={item => setSelectedId(item.id)} />
    </Box>
  );
}
