import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';

export type Tab = {readonly id: string; readonly label: string};

/** large: two rows, a thin ▔ under the active label (top navigation). small: one row, the active label filled (inside pages). */
export type TabsSize = 'large' | 'small';

type Props = {
  tabs: readonly Tab[];
  activeId: string;
  focused?: boolean;
  size?: TabsSize;
  /** large: the background of the label row (the header's surface). */
  labelSurface?: string;
  onChange?: (tab: Tab) => void;
};

/** The thin line hugging the active label from below. */
export const OVERLINE = '▔';

/** Tabs in two sizes. The active tab is the cursor: accent while the tabs are focused, muted/raised otherwise. Click selects. */
export function Tabs({tabs, activeId, focused = true, size = 'small', labelSurface, onChange}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const widths = tabs.map(tab => TextWidth.of(tab.label) + 4);
  useMouseTarget(box, {
    onPress: event => {
      let x = event.localX;
      for (const [index, width] of widths.entries()) {
        if (x < width) return onChange?.(tabs[index]!);
        x -= width;
      }
    },
  });
  if (size === 'small') {
    return (
      <Box ref={box} flexDirection="row" height={1}>
        {tabs.map(tab => {
          const active = tab.id === activeId;
          const highlighted = active && focused;
          const fill = active ? (focused ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised) : undefined;
          return (
            <Box key={tab.id} paddingX={2} flexShrink={0} backgroundColor={fill}>
              <Text bold={active} color={highlighted ? theme.tokens.selectionText : active ? theme.tokens.textBright : theme.tokens.textMuted}>{tab.label}</Text>
            </Box>
          );
        })}
      </Box>
    );
  }
  return (
    <Box ref={box} flexDirection="row" height={2}>
      {tabs.map((tab, index) => {
        const active = tab.id === activeId;
        const width = widths[index]!;
        return (
          <Box key={tab.id} flexDirection="column" width={width} flexShrink={0}>
            <Box height={1} paddingX={2} backgroundColor={labelSurface}>
              <Text bold={active} color={active ? theme.tokens.textBright : theme.tokens.textMuted}>{tab.label}</Text>
            </Box>
            <Box height={1} paddingX={1}>
              <Text color={focused ? theme.tokens.accent : theme.tokens.textMuted}>{active ? OVERLINE.repeat(width - 2) : ''}</Text>
            </Box>
          </Box>
        );
      })}
    </Box>
  );
}
