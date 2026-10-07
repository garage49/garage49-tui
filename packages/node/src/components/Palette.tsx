import {Box, Text} from 'ink';
import {useState} from 'react';
import {Selection} from './Selection.js';
import {useKeys} from '../input/Keys.js';
import {useTheme} from '../theme/ThemeContext.js';
import {List, type ListItem} from './List.js';
import {Overlay} from './Overlay.js';

type Props = {
  title: string;
  items: readonly ListItem[];
  onPick: (item: ListItem) => void;
  onClose: () => void;
};

/** Command palette: a search line followed by the filtered, sectioned list. Owns its query and cursor. */
export function Palette({title, items, onPick, onClose}: Props) {
  const theme = useTheme();
  const [query, setQuery] = useState('');
  const [selectedId, setSelectedId] = useState<string | undefined>();
  const needle = query.trim().toLowerCase();
  const visible = needle ? items.filter(item => item.label.toLowerCase().includes(needle)) : items;
  const current = Selection.ensure(visible, selectedId);

  useKeys([
    {keys: ['esc'], run: onClose},
    {keys: ['up'], run: () => setSelectedId(id => Selection.move(visible, Selection.ensure(visible, id), -1))},
    {keys: ['down'], run: () => setSelectedId(id => Selection.move(visible, Selection.ensure(visible, id), 1))},
    {keys: ['enter'], run: () => { const item = visible.find(candidate => candidate.id === current); if (item) onPick(item); }},
    {keys: ['backspace', 'delete'], run: () => setQuery(q => q.slice(0, -1))},
  ], {onText: text => setQuery(q => q + text)});

  return (
    <Overlay title={title}>
      <Box>
        <Text color={query ? theme.tokens.text : theme.tokens.textMuted}>{query || 'Search'}█</Text>
      </Box>
      <Box height={1} />
      <List items={visible} selectedId={current} onClick={onPick} />
    </Overlay>
  );
}
