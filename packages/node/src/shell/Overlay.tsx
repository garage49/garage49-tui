import {useOverlay} from '../components/Screen.js';

/** Whether an overlay is open: regions stop reacting to keys and drop their focus bar while one is. */
export function useOverlayState(): boolean {
  return useOverlay().isOpen;
}
