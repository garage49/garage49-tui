import {Box} from 'ink';
import {useState} from 'react';
import {Form, Label, List, Section, Select, Split, TextField, Toggle, useKeys} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';
import type {PageProps} from './Page.js';

const fields = ['name', 'url', 'branch', 'auto'] as const;
type Field = (typeof fields)[number];

const branches: readonly ListItem[] = [{id: 'main', label: 'main'}, {id: 'release', label: 'release/0.2'}];
const recent: readonly ListItem[] = [
  {id: 'r1', label: 'garage49-tui', value: '2 min ago', section: 'Recent'},
  {id: 'r2', label: 'herdr-ranch', value: '1 h ago', section: 'Recent'},
  {id: 'r3', label: 'adoc', value: 'yesterday', section: 'Recent'},
];

/** Section, Split and Form together: the layout every page is composed of. */
export function LayoutPage({focused, report}: PageProps) {
  const [field, setField] = useState<Field>('name');
  const [name, setName] = useState('garage49-tui');
  const [url, setUrl] = useState('https://github.com/garage49/garage49-tui');
  const [branch, setBranch] = useState('main');
  const [auto, setAuto] = useState(true);
  const step = (delta: number) => setField(fields[Math.min(fields.length - 1, Math.max(0, fields.indexOf(field) + delta))]!);
  useKeys([{keys: ['up'], run: () => step(-1)}, {keys: ['down'], run: () => step(1)}], {isActive: focused});
  const is = (candidate: Field) => focused && field === candidate;
  return (
    <Box flexDirection="column" flexGrow={1}>
      <Section title="A section">
        <Label variant="muted">A page is sections; a section is a heading and its body; one blank row between sections.</Label>
      </Section>
      <Section title="A form">
        <Label variant="muted">One label column (as wide as the longest label), one value column: the URL field is as wide as the name field.</Label>
        <Form>
          <TextField label="Name" value={name} onChange={setName} focused={is('name')} onFocus={() => setField('name')} />
          <TextField label="Repository URL" value={url} onChange={setUrl} focused={is('url')} onFocus={() => setField('url')} />
          <Select label="Branch" options={branches} value={branch} onChange={o => { setBranch(o.id); report(`branch: ${o.label}`); }} focused={is('branch')} onFocus={() => setField('branch')} />
          <Toggle label="Auto-sync" value={auto} onChange={setAuto} focused={is('auto')} onFocus={() => setField('auto')} />
        </Form>
      </Section>
      <Section title="A split">
        <Label variant="muted">Sub-panes on alternating surfaces, one cell apart, padded 2×1.</Label>
      </Section>
      <Split>
        <Box flexDirection="column">
          <Label variant="heading">Left pane</Label>
          <List items={recent} selectedId="r1" focused={false} />
        </Box>
        <Box flexDirection="column">
          <Label variant="heading">Right pane</Label>
          <Label>The right pane sits on the panel surface, so the edge shows without a line.</Label>
        </Box>
      </Split>
    </Box>
  );
}
