import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {Label, Select, TextField, Toggle, useKeys, useMouseSwitch, opencodeTheme, systemTheme, useTheme, useThemeSwitch} from '@garage49/garage49-tui-ink';
import type {ListItem} from '@garage49/garage49-tui-ink';

const fields = ['theme', 'mouse', 'vim', 'minsize'] as const;
type Field = (typeof fields)[number];

const themes: readonly ListItem[] = [
  {id: 'opencode', label: 'opencode (truecolor)'},
  {id: 'system', label: 'system (ANSI 16)'},
];

/** Preferences: the theme select switches the live theme; the rest is reported. */
export function SettingsPage({focused, report}: PageProps) {
  const theme = useTheme();
  const switchTheme = useThemeSwitch();
  const mouse = useMouseSwitch();
  const [field, setField] = useState<Field>('theme');
  const [vim, setVim] = useState(true);
  const [minSize, setMinSize] = useState('80×24');

  const index = fields.indexOf(field);
  useKeys([
    {keys: ['up'], run: () => setField(fields[Math.max(0, index - 1)]!)},
    {keys: ['down'], run: () => setField(fields[Math.min(fields.length - 1, index + 1)]!)},
  ], {isActive: focused});

  const is = (candidate: Field) => focused && field === candidate;
  return (
    <Box flexDirection="column" width={60}>
      <Label variant="heading">Appearance</Label>
      <Select
        label="Theme"
        options={themes}
        value={theme.name.replace('-dimmed', '')}
        onChange={option => { switchTheme(option.id === 'system' ? systemTheme : opencodeTheme); report(`theme: ${option.label}`); }}
        focused={is('theme')}
        onFocus={() => setField('theme')}
      />
      <Box height={1} />
      <Label variant="heading">Input</Label>
      <Toggle label="Mouse" value={mouse.enabled} onChange={value => { mouse.setEnabled(value); report(`mouse: ${value ? 'on' : 'off'}`); }} focused={is('mouse')} onFocus={() => setField('mouse')} />
      <Toggle label="Vim keys" value={vim} onChange={value => { setVim(value); report(`vim keys: ${value ? 'on' : 'off'}`); }} focused={is('vim')} onFocus={() => setField('vim')} />
      <Box height={1} />
      <Label variant="heading">Window</Label>
      <TextField label="Minimum size" value={minSize} onChange={setMinSize} focused={is('minsize')} onFocus={() => setField('minsize')} />
      <Box height={1} />
      <Label variant="muted">Theme and mouse are live; the other values are only reported. With the mouse on, shift+drag selects text in most terminals; `m` toggles it anywhere.</Label>
    </Box>
  );
}
