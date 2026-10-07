import React from 'react';
import {Box, Text} from 'ink';
import {render} from 'ink-testing-library';
const h = React.createElement;
function Pane({title, focused, children}) {
  const [l, m, r, style] = focused ? ['┏', '━', '┓', 'bold'] : ['┌', '─', '┐', 'single'];
  return h(Box, {flexDirection: 'column', width: '100%'},
    h(Box, {flexDirection: 'row', height: 1},
      h(Box, {flexShrink: 0}, h(Text, null, `${l}${m} `), h(Text, {bold: true}, title), h(Text, null, ' ')),
      h(Box, {flexGrow: 1, flexBasis: 0, overflow: 'hidden', height: 1}, h(Text, {wrap: 'wrap'}, m.repeat(300))),
      h(Box, {flexShrink: 0}, h(Text, null, r))),
    h(Box, {borderStyle: style, borderTop: false, flexDirection: 'column'}, children));
}
const app = h(Box, {width: 40, flexDirection: 'row'},
  h(Box, {width: 20}, h(Pane, {title: 'Projects', focused: true}, h(Text, null, '▸ skills'), h(Text, null, '  한글 aterm'))),
  h(Box, {width: 20}, h(Pane, {title: 'Detail', focused: false}, h(Text, null, 'name skills'))));
const {lastFrame} = render(app);
console.log(lastFrame());
