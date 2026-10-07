import {Gallery} from './app/Gallery.js';
import {run} from '@garage49/garage49-tui-ink';

/** Entry point: the gallery, mounted by the shell with the detected theme. */
class Main {
  static main(): void {
    run(<Gallery />);
  }
}

Main.main();
