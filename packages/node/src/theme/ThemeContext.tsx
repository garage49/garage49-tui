import {createContext, useContext, useState, type ReactNode} from 'react';
import {Theme, opencodeTheme} from './Theme.js';

const ThemeContext = createContext<Theme>(opencodeTheme);
const SwitchContext = createContext<(theme: Theme) => void>(() => {});

export function ThemeProvider({theme, children}: {theme: Theme; children: ReactNode}) {
  return <ThemeContext.Provider value={theme}>{children}</ThemeContext.Provider>;
}

/** Holds the app's theme as state so a settings screen can switch it; wraps the whole app once. */
export function ThemeRoot({initial, children}: {initial: Theme; children: ReactNode}) {
  const [theme, setTheme] = useState(initial);
  return (
    <SwitchContext.Provider value={setTheme}>
      <ThemeContext.Provider value={theme}>{children}</ThemeContext.Provider>
    </SwitchContext.Provider>
  );
}

export function useTheme(): Theme {
  return useContext(ThemeContext);
}

/** The setter installed by ThemeRoot. */
export function useThemeSwitch(): (theme: Theme) => void {
  return useContext(SwitchContext);
}
