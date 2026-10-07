import {describe, expect, it} from 'vitest';
import {ScreenFit} from '../src/shell/Fit.js';

describe('ScreenFit', () => {
  it('keeps everything at a normal size', () => {
    expect(ScreenFit.decide(110, 32, true)).toEqual({main: true, statusLine: true, keyHints: true});
  });

  it('keeps only the sidebar when narrow, but the main page when there is no sidebar', () => {
    expect(ScreenFit.decide(60, 40, true).main).toBe(false);
    expect(ScreenFit.decide(60, 40, false).main).toBe(true);
  });

  it('drops the key hints below 12 rows and the status line below 8', () => {
    expect(ScreenFit.decide(110, 11, false)).toMatchObject({keyHints: false, statusLine: true});
    expect(ScreenFit.decide(110, 7, false)).toMatchObject({keyHints: false, statusLine: false});
  });
});
