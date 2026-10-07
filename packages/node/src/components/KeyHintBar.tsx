import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {TextWidth} from '../theme/TextWidth.js';
import {useTheme} from '../theme/ThemeContext.js';

export type KeyHint = {readonly key: string; readonly label: string};

type Props = {hints: readonly KeyHint[]; left?: string; onPress?: (hint: KeyHint) => void};

/** Bottom line: muted context on the left, "key (bright) + label (muted)" pairs on the right. A click on a pair triggers it. */
export function KeyHintBar({hints, left = '', onPress}: Props) {
  const theme = useTheme();
  const pairs = useRef<DOMElement>(null);
  const widths = hints.map(hint => 2 + TextWidth.of(hint.key) + 1 + TextWidth.of(hint.label));
  useMouseTarget(pairs, {
    onPress: event => {
      let x = event.localX;
      for (const [index, width] of widths.entries()) {
        if (x < width) return onPress?.(hints[index]!);
        x -= width;
      }
    },
  });
  return (
    <Box flexDirection="row" height={1} flexShrink={0} width="100%">
      <Box flexGrow={1} flexBasis={0} overflow="hidden"><Text color={theme.tokens.textMuted} wrap="truncate-middle">{left}</Text></Box>
      <Box ref={pairs} flexShrink={0} flexDirection="row">
        {hints.map(hint => (
          <Box key={hint.key} marginLeft={2}>
            <Text color={theme.tokens.text}>{hint.key} </Text>
            <Text color={theme.tokens.textMuted}>{hint.label}</Text>
          </Box>
        ))}
      </Box>
    </Box>
  );
}
