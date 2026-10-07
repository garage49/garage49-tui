import {Box, Text, type DOMElement} from 'ink';
import {useRef, type ReactNode} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {useTyping} from '../shell/Typing.js';
import {useTheme} from '../theme/ThemeContext.js';

type Props = {
  value: string;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  placeholder?: string;
  focused?: boolean;
  /** Mouse click inside the field. */
  onFocus?: () => void;
  /** Secondary line under the text, e.g. mode and model. */
  footer?: ReactNode;
};

/**
 * OpenCode's input: a surface behind the text with a half-block bottom edge; while focused a blue ┃
 * is painted over the surface's first column (the FocusRegion rule at field scale).
 */
export function Input({value, onChange, onSubmit, placeholder = '', focused = true, onFocus, footer}: Props) {
  const theme = useTheme();
  useTyping(focused);
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: () => onFocus?.()});
  useKeys([
    {keys: ['enter'], run: () => onSubmit?.(value)},
    {keys: ['backspace', 'delete'], run: () => onChange?.(value.slice(0, -1))},
  ], {isActive: focused, onText: text => onChange?.(value + text)});
  const cursor = focused ? '█' : '';
  const bar = focused ? '┃' : ' ';
  return (
    <Box ref={box} flexDirection="column">
      <Box flexDirection="row">
        <Box flexDirection="column" flexShrink={0} width={1} backgroundColor={theme.tokens.surface}>
          <Text color={theme.tokens.accentSecondary}>{bar}</Text>
          <Text color={theme.tokens.accentSecondary}>{bar}</Text>
          <Text color={theme.tokens.accentSecondary}>{bar}</Text>
          {footer && <Text color={theme.tokens.accentSecondary}>{bar}</Text>}
        </Box>
        <Box flexDirection="column" flexGrow={1} backgroundColor={theme.tokens.surface} paddingX={2}>
          <Text> </Text>
          <Text color={value ? theme.tokens.text : theme.tokens.textMuted} wrap="truncate-end">
            {value || placeholder}{cursor}
          </Text>
          <Text> </Text>
          {footer}
        </Box>
      </Box>
      <Box flexDirection="row">
        <Text color={focused ? theme.tokens.accentSecondary : theme.tokens.surface}>{focused ? '╹' : '▀'}</Text>
        <Box flexGrow={1} overflow="hidden" height={1}>
          <Text color={theme.tokens.surface} wrap="wrap">{'▀'.repeat(400)}</Text>
        </Box>
      </Box>
    </Box>
  );
}
