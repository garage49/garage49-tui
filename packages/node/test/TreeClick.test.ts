import {describe, expect, it} from 'vitest';
import {TreeClick} from '../src/components/TreeClick.js';

describe('TreeClick', () => {
  it('selects an unselected row and toggles the marker or a selected row', () => {
    expect(TreeClick.decide(false, false)).toBe('select');
    expect(TreeClick.decide(true, false)).toBe('toggle');
    expect(TreeClick.decide(false, true)).toBe('toggle');
  });

  it('finds the marker cell after the padding and the indent', () => {
    expect(TreeClick.isMarker(1, 0)).toBe(true);
    expect(TreeClick.isMarker(3, 1)).toBe(true);
    expect(TreeClick.isMarker(4, 1)).toBe(false);
  });
});
