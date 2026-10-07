import {createContext, useContext, useState, type ReactNode} from 'react';

type StatusState = {readonly lastAction: string; report: (text: string) => void};

const StatusContext = createContext<StatusState>({lastAction: 'ready', report: () => {}});

export function StatusProvider({children}: {children: ReactNode}) {
  const [lastAction, setLastAction] = useState('ready');
  return <StatusContext.Provider value={{lastAction, report: setLastAction}}>{children}</StatusContext.Provider>;
}

/** The status line's "last action" slot: report('saved') shows it at the bottom left. */
export function useStatus(): StatusState {
  return useContext(StatusContext);
}
