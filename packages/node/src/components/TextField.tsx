import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useEditing} from './Editing.js';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {useTyping} from '../shell/Typing.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';
import {useFieldColumns} from './Form.js';

type Props = {
  label: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  focused?: boolean;
  onFocus?: () => void;
  labelWidth?: number;
  /** Show * for every character (tokens, passwords). */
  secret?: boolean;
};

/** A one-line form field: label on the left, the value on a surface; the focused field gets the accent bar. */
export function TextField({label, value, onChange, placeholder = '', focused = false, onFocus, labelWidth, secret = false}: Props) {
  const theme = useTheme();
  const {labelCol} = useFieldColumns(labelWidth);
  const box = useRef<DOMElement>(null);
  const {editing, resume} = useEditing(box, focused);
  useTyping(editing);
  useMouseTarget(box, {onPress: () => { resume(); onFocus?.(); }});
  // Keys faster than renders chain on `pending`, not on the value captured at the last render.
  const pending = useRef(value);
  pending.current = value;
  const emit = (next: string) => {
    pending.current = next;
    onChange?.(next);
  };
  useKeys([
    {keys: ['backspace', 'delete'], run: () => emit(pending.current.slice(0, -1))},
  ], {isActive: editing, onText: text => emit(pending.current + text)});
  return (
    <Box ref={box} flexDirection="row" height={1}>
      <Box width={labelCol} flexShrink={0}><Text color={editing ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={editing} />
      <Box flexGrow={1} backgroundColor={theme.tokens.surface} paddingX={1}>
        <Text color={value ? theme.tokens.text : theme.tokens.textMuted} wrap="truncate-end">{shownText(value, placeholder, secret)}{editing ? '█' : ''}</Text>
      </Box>
    </Box>
  );
}

/** The field's visible text: the placeholder while empty, stars for a secret, the value otherwise. */
function shownText(value: string, placeholder: string, secret: boolean): string {
  if (!value) return placeholder;
  return secret ? '*'.repeat([...value].length) : value;
}
