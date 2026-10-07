import {Box, Text, type DOMElement} from 'ink';
import {useRef, useState} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';
import type {ListItem} from './List.js';

type Props = {
  label: string;
  options: readonly ListItem[];
  value: string;
  onChange?: (option: ListItem) => void;
  focused?: boolean;
  onFocus?: () => void;
  labelWidth?: number;
  /** ↑ on the first option or ↓ on the last: hand the focus to the neighbouring field. */
  onLeave?: (direction: -1 | 1) => void;
};

/**
 * One field with one row per option: the on glyph for the chosen one in the success color, the off glyph muted otherwise.
 * ↑↓ move the cursor (the accent-filled row); space or enter chooses; a click chooses directly.
 */
export function RadioGroup({label, options, value, onChange, focused = false, onFocus, labelWidth = 14, onLeave}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const [cursor, setCursor] = useState(() => Math.max(0, options.findIndex(option => option.id === value)));
  const choose = (index: number) => {
    const option = options[index];
    if (option) onChange?.(option);
  };
  useMouseTarget(box, {
    onPress: event => {
      onFocus?.();
      if (event.localY < options.length) {
        setCursor(event.localY);
        choose(event.localY);
      }
    },
  });
  useKeys([
    {keys: ['up', 'k'], run: () => (cursor === 0 ? onLeave?.(-1) : setCursor(cursor - 1))},
    {keys: ['down', 'j'], run: () => (cursor === options.length - 1 ? onLeave?.(1) : setCursor(cursor + 1))},
    {keys: ['space', 'enter'], run: () => choose(cursor)},
  ], {isActive: focused});
  return (
    <Box ref={box} flexDirection="row" height={options.length}>
      <Box width={labelWidth} flexShrink={0}><Text color={focused ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={focused} surface="background" rows={options.length} />
      <Box flexDirection="column">
        {options.map((option, index) => {
          const chosen = option.id === value;
          const highlighted = focused && index === cursor;
          const fill = highlighted ? theme.tokens.selectionBackground : undefined;
          const mark = highlighted ? theme.tokens.selectionText : chosen ? theme.tokens.success : theme.tokens.textMuted;
          return (
            <Box key={option.id} flexDirection="row" backgroundColor={fill} paddingX={1}>
              <Text bold color={mark}>{chosen ? Glyphs.on : Glyphs.off} </Text>
              <Text bold={highlighted} color={highlighted ? theme.tokens.selectionText : chosen || focused ? theme.tokens.text : theme.tokens.textMuted}>{option.label}</Text>
            </Box>
          );
        })}
      </Box>
    </Box>
  );
}
