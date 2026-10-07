import {Box, Text, type DOMElement} from 'ink';
import {useRef} from 'react';
import {useMouseTarget} from '../input/Mouse.js';
import {useTheme} from '../theme/ThemeContext.js';

export type ChipTone = 'default' | 'accent' | 'success' | 'warning' | 'error';

type Props = {
  label: string;
  /** Colors the label; the pill stays on the raised surface so tone reads as a state, not a cursor. */
  tone?: ChipTone;
  /** The cursor is on this chip: filled with the accent color. */
  selected?: boolean;
  /** Shows a × and calls onRemove when it is clicked. */
  onRemove?: () => void;
  onPress?: () => void;
};

/** A small pill on the raised surface: a tag, a filter, a status. Click selects; × removes. */
export function Chip({label, tone = 'default', selected = false, onRemove, onPress}: Props) {
  const theme = useTheme();
  const box = useRef<DOMElement>(null);
  const tones: Record<ChipTone, string> = {
    default: theme.tokens.text,
    accent: theme.tokens.accent,
    success: theme.tokens.success,
    warning: theme.tokens.warning,
    error: theme.tokens.error,
  };
  useMouseTarget(box, {
    onPress: event => {
      if (onRemove && event.localX >= [...label].length + 2) return onRemove();
      onPress?.();
    },
  });
  return (
    <Box ref={box} paddingX={1} marginRight={1} backgroundColor={selected ? theme.tokens.selectionBackground : theme.tokens.surfaceRaised}>
      <Text bold={selected} color={selected ? theme.tokens.selectionText : tones[tone]}>{label}</Text>
      {onRemove && <Text color={selected ? theme.tokens.selectionText : theme.tokens.textMuted}> ×</Text>}
    </Box>
  );
}
