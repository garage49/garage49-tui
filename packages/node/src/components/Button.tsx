import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';

type Props = {label: string; selected?: boolean; onPress?: () => void};

/** A clickable label on a raised surface; the selected button is filled with the accent color. */
export function Button({label, selected = false, onPress}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: event => { if (event.button === 'left') onPress?.(); }});
  return (
    <Box ref={box} backgroundColor={selected ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised} paddingX={2} marginLeft={2}>
      <Text bold={selected} color={selected ? theme.tokens.selectionText : theme.tokens.text}>{label}</Text>
    </Box>
  );
}
