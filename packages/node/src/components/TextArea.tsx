import {Box, Text, type DOMElement} from 'ink';
import {useEffect, useRef, useState} from 'react';
import {useKeys, type Binding} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {useTyping} from '../shell/Typing.js';
import {useTheme} from '../theme/ThemeContext.js';
import {useEditing} from './Editing.js';
import {FieldBar} from './FieldBar.js';
import {useFieldColumns} from './Form.js';
import {Scrollbar} from './Scrollbar.js';
import {TextBuffer} from './TextBuffer.js';

type Props = {
  label: string;
  buffer: TextBuffer;
  onChange?: (buffer: TextBuffer) => void;
  placeholder?: string;
  focused?: boolean;
  onFocus?: () => void;
  labelWidth?: number;
  /** Visible rows; the view scrolls to keep the cursor inside. */
  rows?: number;
  /** ↑ on the first line or ↓ on the last line: hand the focus to the neighbouring field. */
  onLeave?: (direction: -1 | 1) => void;
};

/**
 * A multi-line editor on a surface: arrows move the cursor, enter splits the line, backspace joins.
 * The cursor is the inverted cell. The wheel scrolls the view; moving the cursor brings it back into view.
 */
export function TextArea({label, buffer, onChange, placeholder = '', focused = false, onFocus, labelWidth, rows = 5, onLeave}: Props) {
  const theme = useTheme();
  const {labelCol} = useFieldColumns(labelWidth);
  const box = useRef<DOMElement>(null);
  const {editing, resume} = useEditing(box, focused);
  useTyping(editing);
  const [scrollTop, setScrollTop] = useState(0);
  const maxTop = Math.max(0, buffer.lines.length - rows);
  const top = Math.min(scrollTop, maxTop);

  useEffect(() => {
    if (buffer.row < top) setScrollTop(buffer.row);
    else if (buffer.row >= top + rows) setScrollTop(buffer.row - rows + 1);
  }, [buffer, rows]); // follows the cursor whenever the buffer changes; `top` is derived from it

  useMouseTarget(box, {
    onPress: () => { resume(); onFocus?.(); },
    onWheel: event => setScrollTop(Math.max(0, Math.min(maxTop, top + (event.kind === 'wheel-up' ? -3 : 3)))),
  });
  // Keys faster than renders chain on `pending`, not on the buffer captured at the last render.
  const pending = useRef(buffer);
  pending.current = buffer;
  const emit = (next: TextBuffer) => {
    pending.current = next;
    onChange?.(next);
  };
  useKeys(editorBindings(() => pending.current, emit, rows, onLeave), {isActive: editing, onText: text => emit(pending.current.insert(text))});

  const empty = buffer.text === '' && !editing;
  return (
    <Box ref={box} flexDirection="row" height={rows}>
      <Box width={labelCol} flexShrink={0}><Text color={editing ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={editing} rows={rows} />
      <Box flexDirection="column" flexGrow={1} backgroundColor={theme.tokens.surface} paddingX={1}>
        {empty && <Text color={theme.tokens.textMuted}>{placeholder}</Text>}
        {!empty && Array.from({length: rows}, (_, index) => {
          const row = top + index;
          return <TextAreaLine key={row} line={buffer.lines[row]} cursorCol={editing && row === buffer.row ? buffer.col : undefined} />;
        })}
      </Box>
      <Box backgroundColor={theme.tokens.surface}><Scrollbar rows={rows} total={buffer.lines.length} visible={rows} offset={top} /></Box>
    </Box>
  );
}

/** One line of the editor; with a cursor column, that cell is drawn inverted. */
function TextAreaLine({line, cursorCol}: {line: string | undefined; cursorCol: number | undefined}) {
  const theme = useTheme();
  if (line === undefined) return <Text> </Text>;
  if (cursorCol === undefined) return <Text color={theme.tokens.text} wrap="truncate-end">{line || ' '}</Text>;
  const chars = [...line];
  return (
    <Text color={theme.tokens.text} wrap="truncate-end">
      {chars.slice(0, cursorCol).join('')}<Text inverse>{chars[cursorCol] ?? ' '}</Text>{chars.slice(cursorCol + 1).join('')}
    </Text>
  );
}

/** The editor's keys over the latest buffer; ↑ on the first line and ↓ on the last leave the field. */
function editorBindings(current: () => TextBuffer, emit: (next: TextBuffer) => void, rows: number, onLeave?: (direction: -1 | 1) => void): Binding[] {
  const edit = (f: (b: TextBuffer) => TextBuffer) => () => emit(f(current()));
  return [
    {keys: ['up'], run: () => (current().row === 0 ? onLeave?.(-1) : emit(current().move(-1, 0)))},
    {keys: ['down'], run: () => (current().row === current().lines.length - 1 ? onLeave?.(1) : emit(current().move(1, 0)))},
    {keys: ['left'], run: edit(b => b.move(0, -1))},
    {keys: ['right'], run: edit(b => b.move(0, 1))},
    {keys: ['pageup'], run: edit(b => b.move(-rows, 0))},
    {keys: ['pagedown'], run: edit(b => b.move(rows, 0))},
    {keys: ['home'], run: edit(b => b.home())},
    {keys: ['end'], run: edit(b => b.end())},
    {keys: ['enter'], run: edit(b => b.newline())},
    {keys: ['backspace', 'delete'], run: edit(b => b.backspace())},
  ];
}
