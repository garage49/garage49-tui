import {Box, Text} from 'ink';
import {render} from 'ink-testing-library';
import {describe, expect, it} from 'vitest';
import {MasterDetail} from '../src/components/MasterDetail.js';

const items = [{id: 'a', label: 'garage49-tui', value: '2 min ago'}, {id: 'b', label: 'adoc', value: 'yesterday'}];

describe('MasterDetail', () => {
  it('puts the list on the left third and the detail, padded, on the right', () => {
    const {lastFrame} = render(
      <Box width={90}>
        <MasterDetail items={items} selectedId="b" focused={false}><Text>Path  ~/work/adoc</Text></MasterDetail>
      </Box>,
    );
    expect(lastFrame()).toMatchSnapshot();
  });
});
