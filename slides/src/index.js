import Reveal from 'reveal.js';
import markdown from 'reveal.js/plugin/markdown/markdown';
import highlight from 'reveal.js/plugin/highlight/highlight';
import notes from 'reveal.js/plugin/notes/notes';
import zoom from 'reveal.js/plugin/zoom/zoom';
import search from 'reveal.js/plugin/search/search';

let deck = new Reveal({
  plugins: [markdown, highlight, notes, zoom, search]
})

deck.initialize({
  hash: true
});
