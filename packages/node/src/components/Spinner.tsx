import {Text, useAnimation} from 'ink';
import {Glyphs} from '../theme/Glyphs.js';
import {useTheme} from '../theme/ThemeContext.js';

export type SpinnerKind = 'dots' | 'line' | 'bounce';

const FRAMES: Record<SpinnerKind, readonly string[]> = {
  dots: ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
  line: ['|', '/', '-', '\\'],
  bounce: ['⠁', '⠂', '⠄', '⠂'],
};

type Props = {kind?: SpinnerKind; label?: string; active?: boolean};

/** An in-progress marker in the accentSecondary color, with an optional muted label. */
export function Spinner({kind = 'dots', label, active = true}: Props) {
  const theme = useTheme();
  const {frame} = useAnimation({interval: 160, isActive: active});
  const frames = FRAMES[kind];
  return (
    <Text>
      <Text color={theme.tokens.accentSecondary}>{active ? frames[frame % frames.length] : Glyphs.on}</Text>
      {label && <Text color={theme.tokens.textMuted}> {label}</Text>}
    </Text>
  );
}
