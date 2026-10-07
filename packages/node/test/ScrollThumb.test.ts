import {describe, expect, it} from 'vitest';
import {ScrollThumb} from '../src/components/Scrollbar.js';

describe('ScrollThumb', () => {
  it('fills the track when everything fits', () => {
    expect(ScrollThumb.place(5, 3, 5, 0)).toEqual({start: 0, size: 5});
  });

  it('sizes the thumb by the visible share and moves it to the end at the maximum offset', () => {
    expect(ScrollThumb.place(6, 12, 6, 0)).toEqual({start: 0, size: 3});
    expect(ScrollThumb.place(6, 12, 6, 6)).toEqual({start: 3, size: 3});
    expect(ScrollThumb.place(6, 60, 6, 27)).toEqual({start: 3, size: 1});
  });
});
