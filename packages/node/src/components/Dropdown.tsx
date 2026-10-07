import {Box} from 'ink';
import {useState} from 'react';
import {Selection} from './Selection.js';
import {useKeys} from '../input/Keys.js';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';
import {List, type ListItem} from './List.js';

type Props = {
  items: readonly ListItem[];
  /** The row the cursor starts on; the first row otherwise. */
  initialId?: string;
  onPick: (item: ListItem) => void;
  onClose: () => void;
  /** Left/right arrow: move to a neighbouring dropdown (menus). Absent, the arrows do nothing. */
  onNavigate?: (delta: -1 | 1) => void;
};

/** A list popover on a raised surface, under a menu label or a select box. Owns its cursor and keys. */
export function Dropdown({items, initialId, onPick, onClose, onNavigate}: Props) {
  const theme = useTheme();
  const [cursorId, setCursorId] = useState<string | undefined>(initialId);
  const cursor = Selection.ensure(items, cursorId);
  useKeys([
    {keys: ['esc'], run: onClose},
    {keys: ['left'], run: () => onNavigate?.(-1)},
    {keys: ['right'], run: () => onNavigate?.(1)},
    {keys: ['up'], run: () => setCursorId(id => Selection.move(items, Selection.ensure(items, id), -1))},
    {keys: ['down'], run: () => setCursorId(id => Selection.move(items, Selection.ensure(items, id), 1))},
    {keys: ['enter'], run: () => { const item = items.find(candidate => candidate.id === cursor); if (item) onPick(item); }},
  ]);
  const widest = TextWidth.widest(items.map(item => item.label + (item.shortcut ? `  ${item.shortcut}` : '')));
  return (
    <Box flexDirection="column" backgroundColor={theme.tokens.surface} width={widest + 4}>
      <List items={items} selectedId={cursor} onClick={onPick} />
    </Box>
  );
}
