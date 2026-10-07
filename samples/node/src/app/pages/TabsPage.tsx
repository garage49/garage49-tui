import {Box} from 'ink';
import {useState} from 'react';
import type {PageProps} from './Page.js';
import {Label, Tabs, useKeys, Section} from '@garage49/garage49-tui-ink';
import type {Tab} from '@garage49/garage49-tui-ink';

const tabs: readonly Tab[] = [
  {id: 'summary', label: 'Summary'},
  {id: 'files', label: 'Files'},
  {id: 'history', label: '기록'},
];

const bodies: Record<string, string> = {
  summary: 'Small tabs fill the active label; large tabs mark it with a thin line. Both: accent while focused.',
  files: 'Tabs switch with ←→ or hl; a click on a tab selects it.',
  history: '한글 탭 제목도 폭이 맞게 그려집니다.',
};

export function TabsPage({focused, report}: PageProps) {
  const [activeId, setActiveId] = useState('summary');
  const select = (tab: Tab) => {
    setActiveId(tab.id);
    report(`tab: ${tab.label}`);
  };
  const index = tabs.findIndex(tab => tab.id === activeId);
  useKeys([
    {keys: ['left', 'h'], run: () => select(tabs[(index + tabs.length - 1) % tabs.length]!)},
    {keys: ['right', 'l'], run: () => select(tabs[(index + 1) % tabs.length]!)},
  ], {isActive: focused});
  return (
    <Box flexDirection="column">
      <Section title="small · inside pages">
        <Tabs tabs={tabs} activeId={activeId} focused={focused} onChange={select} />
        <Box paddingX={2} paddingY={1}><Label>{bodies[activeId]}</Label></Box>
      </Section>
      <Section title="large · the top navigation">
        <Tabs tabs={tabs} activeId={activeId} focused={focused} size="large" onChange={select} />
      </Section>
    </Box>
  );
}
