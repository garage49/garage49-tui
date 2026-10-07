import {Box} from 'ink';
import type {PageProps} from './Page.js';
import {Label} from '@garage49/garage49-tui-ink';

export function HomePage(_: PageProps) {
  return (
    <Box flexDirection="column">
      <Label variant="bright">garage49 TUI design system</Label>
      <Label variant="muted">Ink sample gallery · every element the system defines, one page each.</Label>
      <Box height={1} />
      <Label variant="heading">How to move</Label>
      <Label>tab        cycle focus: navigation → sidebar → page; click a region to focus it</Label>
      <Label>enter/esc  go down into the view / back up</Label>
      <Label>↑↓ / jk    move in the sidebar or in the page's list</Label>
      <Label>←→ / hl    switch the view while the top navigation is focused</Label>
      <Label>ctrl+p     command palette · t toggle theme · ? help · q quit</Label>
      <Label>mouse      click rows, tabs, menus, buttons and key hints; wheel scrolls lists and logs</Label>
      <Box height={1} />
      <Label variant="heading">Surfaces</Label>
      <Label variant="muted">No line borders. Regions are background colors; edges are half blocks; focus is a left accent bar.</Label>
    </Box>
  );
}
