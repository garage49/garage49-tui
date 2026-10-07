import {describe, expect, it} from 'vitest';
import {TreeLayout} from '../src/components/TreeView.js';

const nodes = [
  {id: 'a', label: 'a', children: [{id: 'a1', label: 'a1'}, {id: 'a2', label: 'a2', children: [{id: 'a2x', label: 'a2x'}]}]},
  {id: 'b', label: 'b'},
];

describe('TreeLayout', () => {
  it('lists only the rows under expanded branches, with depth and parent', () => {
    const rows = TreeLayout.rows(nodes, new Set(['a']));
    expect(rows.map(row => [row.node.id, row.depth, row.parentId])).toEqual([
      ['a', 0, undefined], ['a1', 1, 'a'], ['a2', 1, 'a'], ['b', 0, undefined],
    ]);
  });

  it('hides a branch that is expanded but whose parent is collapsed', () => {
    expect(TreeLayout.rows(nodes, new Set(['a2'])).map(row => row.node.id)).toEqual(['a', 'b']);
  });
});

describe('TreeLayout.cursorAfterCollapse', () => {
  it('moves a cursor that would vanish to the collapsing branch and leaves others alone', () => {
    expect(TreeLayout.cursorAfterCollapse(nodes, 'a', 'a2x')).toBe('a');
    expect(TreeLayout.cursorAfterCollapse(nodes, 'a', 'a')).toBe('a');
    expect(TreeLayout.cursorAfterCollapse(nodes, 'a', 'b')).toBe('b');
    expect(TreeLayout.cursorAfterCollapse(nodes, 'a2', 'a1')).toBe('a1');
  });
});
