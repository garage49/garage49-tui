import {Box, Text, measureElement, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';
import {useFieldColumns} from './Form.js';
import {Dropdown} from './Dropdown.js';
import type {ListItem} from './List.js';
import {useOverlay} from './Screen.js';

type Props = {
  label: string;
  options: readonly ListItem[];
  value: string;
  onChange?: (option: ListItem) => void;
  focused?: boolean;
  onFocus?: () => void;
  labelWidth?: number;
  width?: number;
};

/** A select box: the current option with a ▾ on a surface. Enter, space or a click opens a dropdown; ←→ cycle without opening. */
export function Select({label, options, value, onChange, focused = false, onFocus, labelWidth, width}: Props) {
  const theme = useTheme();
  const {labelCol, valueCol} = useFieldColumns(labelWidth, width);
  const overlay = useOverlay();
  const box = useRef<DOMElement>(null);
  const current = options.find(option => option.id === value);
  const index = options.findIndex(option => option.id === value);

  const open = () => {
    if (!box.current) return;
    const rect = measureElement(box.current);
    const labelWidth = labelCol;
    overlay.show(
      <Dropdown items={options} initialId={value} onClose={overlay.hide} onPick={option => { overlay.hide(); onChange?.(option); }} />,
      {top: rect.y + 1, left: rect.x + labelWidth},
    );
  };
  const cycle = (delta: number) => {
    const next = options[(index + delta + options.length) % options.length];
    if (next) onChange?.(next);
  };

  useMouseTarget(box, {onPress: () => { onFocus?.(); open(); }});
  useKeys([
    {keys: ['enter', 'space'], run: open},
    {keys: ['left'], run: () => cycle(-1)},
    {keys: ['right'], run: () => cycle(1)},
  ], {isActive: focused});

  return (
    <Box ref={box} flexDirection="row" height={1}>
      <Box width={labelCol} flexShrink={0}><Text color={focused ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={focused} />
      <Box width={valueCol} backgroundColor={theme.tokens.surface} paddingX={1} flexDirection="row">
        <Box flexGrow={1} overflow="hidden"><Text color={theme.tokens.text} wrap="truncate-end">{current?.label ?? ''}</Text></Box>
        <Text color={focused ? theme.tokens.text : theme.tokens.textMuted}> ▾</Text>
      </Box>
    </Box>
  );
}
