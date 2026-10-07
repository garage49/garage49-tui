import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {useTyping} from '../shell/Typing.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';

type Props = {
  label: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  focused?: boolean;
  onFocus?: () => void;
  labelWidth?: number;
};

/** A one-line form field: label on the left, the value on a surface; the focused field gets the accent bar. */
export function TextField({label, value, onChange, placeholder = '', focused = false, onFocus, labelWidth = 14}: Props) {
  const theme = useTheme();
  useTyping(focused);
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: () => onFocus?.()});
  useKeys([
    {keys: ['backspace', 'delete'], run: () => onChange?.(value.slice(0, -1))},
  ], {isActive: focused, onText: text => onChange?.(value + text)});
  return (
    <Box ref={box} flexDirection="row" height={1}>
      <Box width={labelWidth} flexShrink={0}><Text color={focused ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={focused} />
      <Box flexGrow={1} backgroundColor={theme.tokens.surface} paddingX={1}>
        <Text color={value ? theme.tokens.text : theme.tokens.textMuted} wrap="truncate-end">{value || placeholder}{focused ? '█' : ''}</Text>
      </Box>
    </Box>
  );
}
