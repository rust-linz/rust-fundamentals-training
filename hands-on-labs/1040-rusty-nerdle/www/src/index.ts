import './index.css';

import { Game } from './game';

// Note: Our initialization function in Rust is marked with #[wasm_bindgen(start)].
// Therefore, it runs automatically when the Wasm module is loaded.
new Game(document.querySelector<HTMLDivElement>('#app'));
