import {render} from 'ink-testing-library';
import {describe, expect, it} from 'vitest';
import {List} from '../src/components/List.js';

const items = [
  {id: 'a', label: 'Switch model', shortcut: 'ctrl+x m', section: 'Suggested'},
  {id: 'b', label: '한글 항목', section: 'Suggested'},
  {id: 'c', label: 'New session', shortcut: 'ctrl+x n', section: 'Session'},
];

describe('List', () => {
  it('renders sections, a blank line between them, and right-aligned shortcuts', () => {
    const {lastFrame} = render(<List items={items} selectedId="b" width={40} />);
    expect(lastFrame()).toMatchSnapshot();
  });
});
