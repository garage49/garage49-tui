import type {DOMElement} from 'ink';
import {useEffect, useState, type RefObject} from 'react';
import {useMouseOutside} from '../input/Mouse.js';

/**
 * Whether a text field is editing: it is the page's current field (`focused`) and no click has landed
 * outside it since. A click outside ends editing (cursor and bar gone, keys back to the app); a click on
 * the field (`resume`) or the field becoming current again starts it anew.
 */
export function useEditing(box: RefObject<DOMElement | null>, focused: boolean): {editing: boolean; resume: () => void} {
  const [blurred, setBlurred] = useState(false);
  useEffect(() => setBlurred(false), [focused]); // becoming (or ceasing to be) the current field starts afresh
  useMouseOutside(box, () => {
    if (focused) setBlurred(true);
  });
  return {editing: focused && !blurred, resume: () => setBlurred(false)};
}
