import {Box, Text} from 'ink';
import type {PageProps} from './Page.js';
import {Label, useTheme} from '@garage49/garage49-tui-ink';
import type {LabelVariant} from '@garage49/garage49-tui-ink';

const variants: readonly LabelVariant[] = ['default', 'muted', 'bright', 'heading', 'accent', 'success', 'warning', 'error'];

export function LabelsPage(_: PageProps) {
  const theme = useTheme();
  const surfaces = ['background', 'panel', 'surface', 'surfaceRaised', 'selectionBackground'] as const;
  return (
    <Box flexDirection="column">
      <Label variant="heading">Label variants</Label>
      {variants.map(variant => (
        <Box key={variant} flexDirection="row">
          <Box width={12}><Label variant="muted">{variant}</Label></Box>
          <Label variant={variant}>The quick brown fox · 빠른 갈색 여우 · 🦊</Label>
        </Box>
      ))}
      <Box height={1} />
      <Label variant="heading">Surfaces ({theme.name})</Label>
      <Box flexDirection="row">
        {surfaces.map(name => (
          <Box key={name} backgroundColor={theme.tokens[name]} paddingX={1} marginRight={1} flexDirection="column">
            <Text color={name === 'selectionBackground' ? theme.tokens.selectionText : theme.tokens.text}>{name}</Text>
            <Text color={name === 'selectionBackground' ? theme.tokens.selectionText : theme.tokens.textMuted}>{theme.tokens[name]}</Text>
          </Box>
        ))}
      </Box>
    </Box>
  );
}
