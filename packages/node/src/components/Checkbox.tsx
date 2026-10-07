import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useKeys} from '../input/Keys.js';
import {useMouseTarget} from '../input/Mouse.js';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';
import {FieldBar} from './FieldBar.js';
import {useSurface} from './FocusRegion.js';

type Props = {label: string; checked: boolean; onChange?: (checked: boolean) => void; focused?: boolean; onFocus?: () => void};

/** A check box on the screen background: ☑ in the success color when checked, ☐ muted otherwise. Space or click toggles. */
export function Checkbox({label, checked, onChange, focused = false, onFocus}: Props) {
  const theme = useTheme();
  const surface = useSurface();
  const box = useRef<DOMElement>(null);
  useMouseTarget(box, {onPress: () => { onFocus?.(); onChange?.(!checked); }});
  useKeys([{keys: ['space', 'enter'], run: () => onChange?.(!checked)}], {isActive: focused});
  return (
    <Box ref={box} flexDirection="row" height={1}>
      <FieldBar focused={focused} surface={surface} />
      <Text> </Text>
      <Text bold color={checked ? theme.tokens.success : theme.tokens.textMuted}>{checked ? Glyphs.checked : Glyphs.unchecked} </Text>
      <Text color={focused ? theme.tokens.text : theme.tokens.textMuted}>{label}</Text>
    </Box>
  );
}
