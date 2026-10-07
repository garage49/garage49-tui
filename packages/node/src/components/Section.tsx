import {Box} from 'ink';
import type {ReactNode} from 'react';
import {Label} from './Label.js';

type Props = {title?: string; children: ReactNode};

/**
 * One titled area of a page. The title is a heading; the body starts right under it; one blank row
 * separates it from the next section. Pages are composed of sections, never of hand-padded boxes.
 */
export function Section({title, children}: Props) {
  return (
    <Box flexDirection="column" marginBottom={1} flexShrink={0}>
      {title && <Label variant="heading">{title}</Label>}
      {children}
    </Box>
  );
}
