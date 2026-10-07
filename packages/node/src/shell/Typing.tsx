import {createContext, useContext, useEffect, useRef, useState, type ReactNode} from 'react';

type TypingState = {readonly typing: boolean; enter: () => () => void};

const TypingContext = createContext<TypingState>({typing: false, enter: () => () => {}});

/** Tracks whether a text field is being edited, so single-letter app keys (q, ?, t, m) stay out of the way. */
export function TypingProvider({children}: {children: ReactNode}) {
  const [count, setCount] = useState(0);
  const enter = useRef(() => {
    setCount(current => current + 1);
    return () => setCount(current => current - 1);
  }).current;
  return <TypingContext.Provider value={{typing: count > 0, enter}}>{children}</TypingContext.Provider>;
}

export function useTypingState(): TypingState {
  return useContext(TypingContext);
}

/** Called by text fields: while `editing`, the app treats printable keys as text. */
export function useTyping(editing: boolean): void {
  const {enter} = useTypingState();
  useEffect(() => (editing ? enter() : undefined), [editing, enter]);
}
