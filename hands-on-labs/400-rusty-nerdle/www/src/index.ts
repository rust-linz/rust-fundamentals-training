import './index.css';

import { Game } from './game';
import { run } from 'nerdle';

// <<< HERE we call our initialization function in Rust
run();
new Game(document.querySelector<HTMLDivElement>('#app'));
