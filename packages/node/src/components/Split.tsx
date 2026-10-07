import {Box} from 'ink';
import {Children, type ReactNode} from 'react';

type Props = {children: ReactNode; direction?: 'row' | 'column'; /** Fill the remaining height (when its sections grow). */ grow?: boolean};

/**
 * Sections side by side (or stacked): each child is a Section and takes an equal share of the space,
 * one cell apart. The split draws nothing of its own — no surface, no padding — so the structure a
 * reader sees is only ever "sections": the header bars and panel blocks are the Sections' own.
 */
export function Split({children, direction = 'row', grow = false}: Props) {
  const items = Children.toArray(children);
  return (
    <Box flexDirection={direction} flexGrow={grow ? 1 : 0} flexShrink={grow ? 1 : 0} gap={direction === 'row' ? 1 : 0} minHeight={0}>
      {items.map((child, index) => (
        <Box key={index} flexDirection="column" flexGrow={1} flexBasis={0} minHeight={0} overflow="hidden">
          {child}
        </Box>
      ))}
    </Box>
  );
}
