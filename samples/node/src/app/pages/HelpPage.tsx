import {Box} from 'ink';
import type {PageProps} from './Page.js';
import {Label, TextWidth} from '@garage49/garage49-tui-ink';

export type HelpEntry = {readonly keys: string; readonly action: string};

type Props = PageProps & {entries: readonly HelpEntry[]};

/** The Help view: the key bindings as a page, the same list the `?` overlay shows. */
export function HelpPage({entries}: Props) {
  const keyWidth = TextWidth.widest(entries.map(entry => entry.keys)) + 2;
  return (
    <Box flexDirection="column">
      <Label variant="heading">Keys (this app)</Label>
      <Label variant="muted">The shell's own keys are under ? · tab cycles, enter/esc go down/up, ctrl+p palette, t theme, m mouse, q quit.</Label>
      <Box height={1} />
      {entries.map(entry => (
        <Box key={entry.keys} flexDirection="row">
          <Box width={keyWidth} flexShrink={0}><Label>{entry.keys}</Label></Box>
          <Label variant="muted">{entry.action}</Label>
        </Box>
      ))}
      <Box height={1} />
      <Label variant="heading">Color roles</Label>
      <Label variant="muted">accent (orange) = the cursor: one per focused region · accentSecondary (blue) = the focused region's bar, shown only where the keys go · success/warning/error = state values</Label>
    </Box>
  );
}
