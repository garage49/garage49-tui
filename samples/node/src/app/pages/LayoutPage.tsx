import {Box} from 'ink';
import {useState} from 'react';
import {Form, Label, List, MasterDetail, Section, Select, Split, TextField, Toggle, useKeys} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';
import type {PageProps} from './Page.js';

const fields = ['name', 'url', 'branch', 'auto', 'projects'] as const;
type Field = (typeof fields)[number];

const branches: readonly ListItem[] = [{id: 'main', label: 'main'}, {id: 'release', label: 'release/0.2'}];
const recent: readonly ListItem[] = [
  {id: 'r1', label: 'garage49-tui', value: '2 min ago'},
  {id: 'r2', label: 'herdr-ranch', value: '1 h ago'},
  {id: 'r3', label: 'adoc', value: 'yesterday'},
];
const shortcuts: readonly ListItem[] = [
  {id: 's1', label: 'Command palette', shortcut: 'ctrl+p'},
  {id: 's2', label: 'Help', shortcut: '?'},
  {id: 's3', label: 'Quit', shortcut: 'q'},
];
const projects: readonly ListItem[] = recent;
const details: Record<string, readonly ListItem[]> = {
  r1: [{id: 'path', label: 'Path', value: '~/work/garage49-tui'}, {id: 'branch', label: 'Branch', value: 'main'}, {id: 'sync', label: 'Last sync', value: '2 min ago'}],
  r2: [{id: 'path', label: 'Path', value: '~/work/herdr-ranch'}, {id: 'branch', label: 'Branch', value: 'tui'}, {id: 'sync', label: 'Last sync', value: '1 h ago'}],
  r3: [{id: 'path', label: 'Path', value: '~/work/adoc'}, {id: 'branch', label: 'Branch', value: 'main'}, {id: 'sync', label: 'Last sync', value: 'yesterday'}],
};

/** Section, Form, Split and MasterDetail together: the layout every page is composed of. */
export function LayoutPage({focused, report}: PageProps) {
  const [field, setField] = useState<Field>('name');
  const [name, setName] = useState('garage49-tui');
  const [url, setUrl] = useState('https://github.com/garage49/garage49-tui');
  const [branch, setBranch] = useState('main');
  const [auto, setAuto] = useState(true);
  const [project, setProject] = useState('r1');
  const step = (delta: number) => {
    if (field === 'projects') {
      const row = projects.findIndex(p => p.id === project) + delta;
      if (row < 0) setField('auto');
      else setProject(projects[Math.min(row, projects.length - 1)]!.id);
      return;
    }
    setField(fields[Math.min(fields.length - 1, Math.max(0, fields.indexOf(field) + delta))]!);
  };
  useKeys([{keys: ['up'], run: () => step(-1)}, {keys: ['down'], run: () => step(1)}], {isActive: focused});
  const is = (candidate: Field) => focused && field === candidate;
  const pick = (item: ListItem) => { setProject(item.id); setField('projects'); report(`project: ${item.label}`); };
  return (
    <Box flexDirection="column" flexGrow={1}>
      <Section title="A form">
        <Label variant="muted">A page is sections. One label column (as wide as the longest label), one value column: the URL field is as wide as the name field.</Label>
        <Form>
          <TextField label="Name" value={name} onChange={setName} focused={is('name')} onFocus={() => setField('name')} />
          <TextField label="Repository URL" value={url} onChange={setUrl} focused={is('url')} onFocus={() => setField('url')} />
          <Select label="Branch" options={branches} value={branch} onChange={o => { setBranch(o.id); report(`branch: ${o.label}`); }} focused={is('branch')} onFocus={() => setField('branch')} />
          <Toggle label="Auto-sync" value={auto} onChange={setAuto} focused={is('auto')} onFocus={() => setField('auto')} />
        </Form>
      </Section>
      <Split>
        <Section title="Recent">
          <List items={recent} focused={false} />
        </Section>
        <Section title="Shortcuts">
          <List items={shortcuts} focused={false} />
        </Section>
      </Split>
      <Section title="Projects">
        <MasterDetail items={projects} selectedId={project} focused={is('projects')} onSelect={pick} onClick={pick}>
          <List items={details[project] ?? []} focused={false} />
        </MasterDetail>
      </Section>
    </Box>
  );
}
