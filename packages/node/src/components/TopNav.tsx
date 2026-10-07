import {Box, Text, type DOMElement} from 'ink';
import {useRef, type ReactNode} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';
import {Tabs, type Tab} from './Tabs.js';

export type NavItem = Tab;

type Props = {
  /** The logo block at the left: bold text on the heading color; its left padding is the focus bar's column. */
  brand: string;
  items: readonly NavItem[];
  activeId: string;
  focused?: boolean;
  onChange?: (item: NavItem) => void;
  /** Text at the right end, e.g. the current user or version. */
  right?: string;
};

/**
 * Web-style header, two rows: the label row on the surface color (logo block, large tabs, context on the
 * right), then a row on the screen background with a thin ▔ under the active tab.
 */
export function TopNav({brand, items, activeId, focused = false, onChange, right}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: () => {}});
  /** A two-row cell of a fixed width, or growing to fill: content on the surface row, nothing below. */
  const cell = (content: ReactNode, width: number | 'grow') => (
    <Box flexDirection="column" width={width === 'grow' ? undefined : width} flexGrow={width === 'grow' ? 1 : 0} flexBasis={width === 'grow' ? 0 : undefined} overflow="hidden">
      <Box height={1} backgroundColor={theme.tokens.surface}>{content}</Box>
      <Box height={1} />
    </Box>
  );
  return (
    <Box ref={box} flexDirection="row" height={2} width="100%">
      {cell(<Box paddingX={2} backgroundColor={theme.tokens.heading}><Text bold color={theme.tokens.background}>{brand}</Text></Box>, TextWidth.of(brand) + 4)}
      {cell(null, 3)}
      <Tabs tabs={items} activeId={activeId} focused={focused} size="large" labelSurface={theme.tokens.surface} onChange={onChange} />
      {cell(null, 'grow')}
      {right && cell(<Box paddingX={2}><Text color={theme.tokens.textMuted}>{right}</Text></Box>, TextWidth.of(right) + 4)}
    </Box>
  );
}
