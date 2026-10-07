import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {Checkbox, Form, Label, RadioGroup, Section, Select, TextField, Toggle, useKeys} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';

const fields = ['name', 'path', 'theme', 'language', 'layout', 'mouse', 'vim', 'snapshot', 'cjk', 'restore'] as const;
type Field = (typeof fields)[number];

const themes: readonly ListItem[] = [
  {id: 'opencode', label: 'opencode'},
  {id: 'system', label: 'system (ANSI 16)'},
  {id: 'light', label: 'light (coming later)'},
];

const languages: readonly ListItem[] = [
  {id: 'ts', label: 'TypeScript'},
  {id: 'rust', label: 'Rust'},
  {id: 'go', label: 'Go'},
  {id: 'ko', label: '한국어'},
];

const layouts: readonly ListItem[] = [
  {id: 'top', label: 'Top navigation only'},
  {id: 'sidebar', label: 'Sidebar only'},
  {id: 'both', label: 'Both · 상단 + 사이드바'},
];

const checkLabels = {snapshot: 'Snapshot tests', cjk: 'CJK width test · 한글 폭', restore: 'Terminal restore test'} as const;

/** One-line form fields: text, select, toggle, check box. ↑↓ moves between them. */
export function FormPage({focused, report}: PageProps) {
  const [field, setField] = useState<Field>('name');
  const [name, setName] = useState('garage49-tui');
  const [path, setPath] = useState('');
  const [theme, setTheme] = useState('opencode');
  const [language, setLanguage] = useState('ts');
  const [layout, setLayout] = useState('both');
  const [mouse, setMouse] = useState(true);
  const [vim, setVim] = useState(true);
  const [checks, setChecks] = useState({snapshot: true, cjk: true, restore: false});

  const step = (delta: number) => setField(fields[Math.min(fields.length - 1, Math.max(0, fields.indexOf(field) + delta))]!);
  useKeys([
    {keys: ['up'], run: () => step(-1)},
    {keys: ['down'], run: () => step(1)},
  ], {isActive: focused && field !== 'layout'}); // the radio group owns ↑↓ between its options and calls onLeave at the edges

  const is = (candidate: Field) => focused && field === candidate;
  return (
    <Box flexDirection="column" flexShrink={0}>
      <Label variant="muted">↑↓ fields (and radio options) · space chooses/toggles/opens · ←→ cycles a select</Label>
      <Box height={1} />
      <Section title="Project">
      <Form>
      <TextField label="Project name" value={name} onChange={setName} focused={is('name')} onFocus={() => setField('name')} />
      <TextField label="Path" value={path} onChange={setPath} placeholder="~/work/…" focused={is('path')} onFocus={() => setField('path')} />
      <Select label="Theme" options={themes} value={theme} onChange={option => { setTheme(option.id); report(`theme: ${option.label}`); }} focused={is('theme')} onFocus={() => setField('theme')} />
      <Select label="Language" options={languages} value={language} onChange={option => { setLanguage(option.id); report(`language: ${option.label}`); }} focused={is('language')} onFocus={() => setField('language')} />
      <RadioGroup label="Layout" options={layouts} value={layout} onChange={option => { setLayout(option.id); report(`layout: ${option.label}`); }} focused={is('layout')} onFocus={() => setField('layout')} onLeave={step} />
      <Toggle label="Mouse" value={mouse} onChange={setMouse} focused={is('mouse')} onFocus={() => setField('mouse')} />
      <Toggle label="Vim keys" value={vim} onChange={setVim} focused={is('vim')} onFocus={() => setField('vim')} />
      </Form>
      </Section>
      <Section title="Tests to run">
      {(['snapshot', 'cjk', 'restore'] as const).map(id => (
        <Checkbox
          key={id}
          label={checkLabels[id]}
          checked={checks[id]}
          onChange={value => { setChecks({...checks, [id]: value}); report(`${id}: ${value ? 'checked' : 'unchecked'}`); }}
          focused={is(id)}
          onFocus={() => setField(id)}
        />
      ))}
      </Section>
    </Box>
  );
}
