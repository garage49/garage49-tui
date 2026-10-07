import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {Input, Label, TextArea, TextBuffer, useKeys, Section} from '@garage49/garage49-tui-ink';

const fields = ['notes', 'command'] as const;
type Field = (typeof fields)[number];

/** Text editing: the multi-line text area and OpenCode's command input. */
export function EditingPage({focused, report}: PageProps) {
  const [field, setField] = useState<Field>('notes');
  const [notes, setNotes] = useState(() => TextBuffer.from(['Multi-line notes.', '한글도 됩니다.', '', 'Arrows move, enter splits, backspace joins.', 'PageUp/PageDown jump a page.', 'The wheel scrolls the view;', 'moving the cursor brings it back.', '', 'line 9', 'line 10', 'line 11', 'line 12 · the end'].join('\n')));
  const [command, setCommand] = useState('');

  const step = (delta: number) => setField(fields[Math.min(fields.length - 1, Math.max(0, fields.indexOf(field) + delta))]!);
  useKeys([
    {keys: ['up'], run: () => step(-1)},
    {keys: ['down'], run: () => step(1)},
  ], {isActive: focused && field !== 'notes'}); // the text area owns the arrows and calls onLeave at its edges

  const is = (candidate: Field) => focused && field === candidate;
  return (
    <Box flexDirection="column" width={68} flexShrink={0}>
      <Section title="Notes">
      <Label variant="muted">↑ on the first line / ↓ on the last line leaves the text area · click focuses</Label>
      <TextArea label="Notes" buffer={notes} onChange={setNotes} rows={6} placeholder="Write several lines…" focused={is('notes')} onFocus={() => setField('notes')} onLeave={step} />
      </Section>
      <Section title="Command">
      <Input
        value={command}
        onChange={setCommand}
        onSubmit={value => { report(`submitted: ${value || '(empty)'}`); setCommand(''); }}
        placeholder="Type a command and press enter…"
        focused={is('command')}
        onFocus={() => setField('command')}
        footer={<Label variant="muted">enter submits · the bar turns blue when focused</Label>}
      />
      </Section>
    </Box>
  );
}
