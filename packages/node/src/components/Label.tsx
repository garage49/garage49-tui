import {Text, type TextProps} from 'ink';
import type {ReactNode} from 'react';
import {useTheme} from '../theme/ThemeContext.js';

export type LabelVariant = 'default' | 'muted' | 'bright' | 'heading' | 'accent' | 'success' | 'warning' | 'error';

type Props = {variant?: LabelVariant; children: ReactNode; wrap?: TextProps['wrap']};

/** Text in one of the theme's semantic roles. Headings are bold. */
export function Label({variant = 'default', children, wrap}: Props) {
  const theme = useTheme();
  const colors: Record<LabelVariant, string> = {
    default: theme.tokens.text,
    muted: theme.tokens.textMuted,
    bright: theme.tokens.textBright,
    heading: theme.tokens.heading,
    accent: theme.tokens.accent,
    success: theme.tokens.success,
    warning: theme.tokens.warning,
    error: theme.tokens.error,
  };
  return <Text color={colors[variant]} bold={variant === 'heading' || variant === 'bright'} wrap={wrap}>{children}</Text>;
}
