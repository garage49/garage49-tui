import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {Section, Table, useKeys} from '@garage49/garage49-tui-ink';
import type {Column} from '@garage49/garage49-tui-ink';

type Project = {readonly name: string; readonly language: string; readonly files: number; readonly status: string};

const projects: readonly Project[] = [
  {name: 'garage49-tui', language: 'TypeScript · Rust', files: 24, status: 'active'},
  {name: '한글 프로젝트 aterm', language: 'TypeScript', files: 1210, status: 'active'},
  {name: 'skills', language: 'Markdown', files: 88, status: 'idle'},
  {name: 'herdr-connect', language: 'Go', files: 310, status: 'archived'},
  {name: 'voice-bridge 🎙', language: 'Python', files: 57, status: 'idle'},
];

const columns: readonly Column<Project>[] = [
  {id: 'name', title: 'Name', width: 24, cell: p => p.name},
  {id: 'language', title: 'Language', width: 18, cell: p => p.language},
  {id: 'files', title: 'Files', width: 6, align: 'right', cell: p => String(p.files)},
  {id: 'status', title: 'Status', width: 10, cell: p => p.status},
];

export function TablePage({focused, report}: PageProps) {
  const [selected, setSelected] = useState<string | undefined>(projects[0]?.name);
  const index = projects.findIndex(p => p.name === selected);
  const move = (delta: number) => setSelected(projects[Math.min(projects.length - 1, Math.max(0, index + delta))]?.name);
  useKeys([
    {keys: ['up', 'k'], run: () => move(-1)},
    {keys: ['down', 'j'], run: () => move(1)},
    {keys: ['enter'], run: () => selected && report(`open: ${selected}`)},
  ], {isActive: focused});
  return (
    <Section title="Projects">
    <Box flexDirection="column">
      <Table
        columns={columns}
        rows={projects}
        rowId={p => p.name}
        selectedId={selected}
        focused={focused}
        onClick={p => { setSelected(p.name); report(`open: ${p.name}`); }}
        onSelect={p => setSelected(p.name)}
      />
    </Box>
    </Section>
  );
}
