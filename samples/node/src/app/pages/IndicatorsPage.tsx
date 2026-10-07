import {Box} from 'ink';
import {useEffect, useState} from 'react';
import type {PageProps} from './Page.js';
import {Chip, Label, Spinner, useKeys, Glyphs} from '@garage49/garage49-tui-ink';

const initialTags = ['typescript', 'rust', '한글', 'ink', 'iocraft', 'tui'];

/** Chips (tags, filters, statuses) and spinners. */
export function IndicatorsPage(props: PageProps) {
  return (
    <Box flexDirection="column">
      <ChipsDemo {...props} />
      <Box height={1} />
      <SpinnersDemo />
    </Box>
  );
}

function ChipsDemo({focused, report}: PageProps) {
  const [tags, setTags] = useState(initialTags);
  const [cursor, setCursor] = useState(0);
  const [filters, setFilters] = useState<ReadonlySet<string>>(new Set(['active']));

  const remove = (tag: string) => {
    setTags(tags.filter(candidate => candidate !== tag));
    setCursor(Math.max(0, Math.min(cursor, tags.length - 2)));
    report(`removed: ${tag}`);
  };
  const toggleFilter = (filter: string) => {
    const next = new Set(filters);
    if (next.has(filter)) next.delete(filter); else next.add(filter);
    setFilters(next);
    report(`filters: ${[...next].join(', ') || 'none'}`);
  };
  useKeys([
    {keys: ['left', 'h'], run: () => setCursor(Math.max(0, cursor - 1))},
    {keys: ['right', 'l'], run: () => setCursor(Math.min(tags.length - 1, cursor + 1))},
    {keys: ['backspace', 'delete', 'x'], run: () => tags[cursor] && remove(tags[cursor]!)},
    {keys: ['enter'], run: () => tags[cursor] && report(`chip: ${tags[cursor]}`)},
  ], {isActive: focused});

  return (
    <Box flexDirection="column">
      <Label variant="heading">Chips · tags</Label>
      <Label variant="muted">←→ moves the cursor · x or backspace removes · click selects, click × removes</Label>
      <Box flexDirection="row" marginTop={1} flexWrap="wrap">
        {tags.map((tag, index) => (
          <Chip key={tag} label={tag} selected={focused && index === cursor} onPress={() => setCursor(index)} onRemove={() => remove(tag)} />
        ))}
      </Box>
      <Box height={1} />
      <Label variant="heading">Chips · filters (click toggles)</Label>
      <Box flexDirection="row" marginTop={1}>
        {['active', 'idle', 'archived'].map(filter => (
          <Chip key={filter} label={`${filters.has(filter) ? Glyphs.on : Glyphs.off} ${filter}`} tone={filters.has(filter) ? 'accent' : 'default'} onPress={() => toggleFilter(filter)} />
        ))}
      </Box>
      <Box height={1} />
      <Label variant="heading">Chips · status tones</Label>
      <Box flexDirection="row" marginTop={1}>
        <Chip label="running" tone="success" />
        <Chip label="degraded" tone="warning" />
        <Chip label="failed" tone="error" />
        <Chip label="v0.1.0" />
      </Box>
    </Box>
  );
}

function SpinnersDemo() {
  const [progress, setProgress] = useState(0);
  useEffect(() => {
    const timer = setInterval(() => setProgress(current => (current + 1) % 100), 150);
    return () => clearInterval(timer);
  }, []);
  return (
    <Box flexDirection="column">
      <Label variant="heading">Spinners</Label>
      <Box flexDirection="column" marginTop={1}>
        <Spinner kind="dots" label="dots · syncing projects…" />
        <Spinner kind="line" label="line · building" />
        <Spinner kind="bounce" label="bounce · waiting for upstream" />
        <Spinner kind="dots" label="inactive · shows the on glyph" active={false} />
        <Box marginTop={1} flexDirection="row">
          <Spinner kind="dots" />
          <Label variant="muted"> with a progress figure: {String(progress).padStart(2, '0')}%</Label>
        </Box>
      </Box>
    </Box>
  );
}
