import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';

type Props = {label: string; value: boolean; onChange?: (value: boolean) => void; focused?: boolean; onFocus?: () => void; labelWidth?: number};

/** A boolean form field: label, then the on glyph + "on" (success color) or the off glyph + "off" (muted); space or click toggles. */
export function Toggle({label, value, onChange, focused = false, onFocus, labelWidth = 14}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: () => { onFocus?.(); onChange?.(!value); }});
  useKeys([{keys: ['space', 'enter'], run: () => onChange?.(!value)}], {isActive: focused});
  return (
    <Box ref={box} flexDirection="row" height={1}>
      <Box width={labelWidth} flexShrink={0}><Text color={focused ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text></Box>
      <FieldBar focused={focused} />
      <Box backgroundColor={theme.tokens.surface} paddingX={1}>
        <Text bold={value} color={value ? theme.tokens.success : theme.tokens.textMuted}>{value ? `${Glyphs.on} on ` : `${Glyphs.off} off`}</Text>
      </Box>
    </Box>
  );
}
