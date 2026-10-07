import {Box} from 'ink';
import {useEffect, useState} from 'react';
import type {PageProps} from './Page.js';
import {Label, LogView, useKeys, Section} from '@garage49/garage49-tui-ink';
import type {LogEntry, LogLevel} from '@garage49/garage49-tui-ink';

const messages: readonly [LogLevel, string][] = [
  ['info', 'Server listening on :8080'],
  ['debug', 'GET /api/projects 200 12ms'],
  ['info', '한글 로그 메시지도 폭이 맞습니다'],
  ['warn', 'Slow query: SELECT * FROM sessions (812ms)'],
  ['debug', 'Cache hit ratio 0.93'],
  ['error', 'Upstream timeout after 30s: projects-sync'],
  ['info', 'Reconnected to upstream'],
];

/** Produces fake log entries on a timer. */
class LogFeed {
  private count = 0;
  next(): LogEntry {
    const [level, message] = messages[this.count % messages.length]!;
    this.count += 1;
    const time = new Date().toTimeString().slice(0, 8);
    return {time, level, message: `${message} #${this.count}`};
  }
}

export function LogPage({focused, report}: PageProps) {
  const [feed] = useState(() => new LogFeed());
  const [entries, setEntries] = useState<readonly LogEntry[]>(() => Array.from({length: 30}, () => feed.next()));
  const [offset, setOffset] = useState(0);
  const [paused, setPaused] = useState(false);

  useEffect(() => {
    if (paused) return;
    const timer = setInterval(() => setEntries(current => [...current.slice(-500), feed.next()]), 700);
    return () => clearInterval(timer);
  }, [paused, feed]);

  useKeys([
    {keys: ['up', 'k'], run: () => setOffset(offset + 1)},
    {keys: ['down', 'j'], run: () => setOffset(Math.max(0, offset - 1))},
    {keys: ['pageup'], run: () => setOffset(offset + 10)},
    {keys: ['pagedown'], run: () => setOffset(Math.max(0, offset - 10))},
    {keys: ['end', 'G'], run: () => setOffset(0)},
    {keys: ['p'], run: () => { setPaused(!paused); report(paused ? 'log: resumed' : 'log: paused'); }},
  ], {isActive: focused});

  return (
    <Section title="Server log" grow>
    <Box flexDirection="column" flexGrow={1} minHeight={0}>
      <Box flexDirection="row">
        <Label variant="muted">{entries.length} entries · </Label>
        <Label variant={paused ? 'warning' : 'success'}>{paused ? 'paused' : 'live'}</Label>
        <Label variant="muted"> · ↑↓ scroll · G follow · p pause</Label>
        {offset > 0 && <Label variant="accent">  ▲ {offset} rows above the end</Label>}
      </Box>
      <Box height={1} />
      <LogView entries={entries} offset={offset} onScroll={setOffset} />
    </Box>
    </Section>
  );
}
