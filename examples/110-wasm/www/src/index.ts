import * as we from 'wasm_examples';
import { memory } from 'wasm_examples/wasm_examples_bg.wasm';

we.run();

const sayHelloBtn = document.getElementById('sayHello') as HTMLButtonElement;
const sayHelloWithMyAlertBtn = document.getElementById('sayHelloWithMyAlert') as HTMLButtonElement;
const workWithNumbersBtn = document.getElementById('workWithNumbers') as HTMLButtonElement;
const panicBtn = document.getElementById('panic') as HTMLButtonElement;
const boolsBtn = document.getElementById('bools') as HTMLButtonElement;
const fibBtn = document.getElementById('fib') as HTMLButtonElement;
const fibResult = document.getElementById('fibResult') as HTMLSpanElement;
const stringsBtn = document.getElementById('strings') as HTMLButtonElement;
const exportedTypeBtn = document.getElementById('exportedType') as HTMLButtonElement;
const jsValueBtn = document.getElementById('jsValue') as HTMLButtonElement;
const serdeWasmBtn = document.getElementById('serdeWasm') as HTMLButtonElement;
const canvas = document.getElementById('canvas') as HTMLCanvasElement;

// Define a global function that will be called by Rust
(<any>window).myAlert = (msg: string) => alert(msg);

// Call functions exported from Rust
sayHelloBtn.onclick = () => we.sayHello();
sayHelloWithMyAlertBtn.onclick = () => we.sayHelloWithMyAlert();

// Demonstrate working with numbers
workWithNumbersBtn.onclick = () => {
    console.log(`The result is ${we.workWithNumbers(0x1ffffffffffffd, 1, 1).toString(16)}`);
    console.log(`The result is ${we.workWithNumbers(0x1ffffffffffffe, undefined, 1).toString(16)}`);
};

panicBtn.onclick = () => {
    try {
        we.panic();
    } catch (ex) {
        console.error(ex);
    }
};

boolsBtn.onclick = () => alert((we.allTrue(true, true) ? "" : "NOT ") + "all are true");

fibBtn.onclick = () => {
    const buffer = new Int32Array(10);
    we.fillWithFibonacci(buffer);
    fibResult.innerText = buffer.join(',');
}

stringsBtn.onclick = () => {
    // Note that the following text contains a paired surrogate char (\ud834\udd1e = 𝄞).
    // It also contains an unpaired surrogate char (\ud834). This is fine in JavaScript.
    // If you pass it to Rust, Rust will replace the unpaired surrogate with \ufffd (�).
    let text = "ABCabc🎵\ud834\udd1e\ud834";

    // Print the text to console and pass-through the text from Rust back to JS.
    // Passing through the text will "destroy" the unpaired surrogate.
    let newText = we.writeToConsole("Original: " + text);

    // It is fine to "fix" the unpaired surrogate based on the JS string.
    // Fixing it based on the Rust string will not work because unpaired surrogate
    // will have been "destroyed".
    text += "\udd1e";
    newText += "\udd1e";
    we.writeToConsole("Fixed (JS): " + text);
    we.writeToConsole("\"Fixed\" (Rust): " + newText);

    // For more information about handling of unpaired surrogates read
    // https://rustwasm.github.io/docs/wasm-bindgen/reference/types/str.html#utf-16-vs-utf-8
}

exportedTypeBtn.onclick = () => {
    let person: we.Person, p: we.Person;
    try {
        // Create person using constructor function.
        // Note that person lives in "WASM land".
        person = we.Person.new('JS', 'Foo', 42);

        // Call method. Method takes ownership of person and will
        // therefore free the associated memory.
        p = we.fixAge(person, 2);
        console.log(`${p.first_name}, ${p.last_name}, ${p.age}`);
    } catch (ex) {
        console.error(ex);
    } finally {
        // Free person returned by fixAge.
        if (p) p.free();
    }
}

jsValueBtn.onclick = () => {
    try {
        // Tip: Try to mess up person object and see error handling in action
        const p = we.fixAgeDynamic({ firstName: 'Foo', lastName: 'Bar', age: 42}, 2);
        console.log(p);
    } catch (ex) {
        console.error(ex);
    }
}

serdeWasmBtn.onclick = () => {
    try {
        // Tip: Try to mess up person object and see error handling in action
        const p = we.fixAgeSerdeWasm({ firstName: 'Foo', lastName: 'Bar', age: 42}, 2);
        console.log(p);
    } catch (ex) {
        console.error(ex);
    }
}

// Create a new display object
const display = we.Display.new(30);

// Get shared buffer and access it via Uint8Array
const pixel =  new Uint8Array(memory.buffer, display.pixel(), 30);

// Regularly redraw canvas
const ctx = canvas.getContext('2d', { alpha: false });
setInterval(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < pixel.length; i++) {
        if (pixel[i] > 0) {
            // Intensity of RED depends on value in shared buffer
            ctx.fillStyle = `hsl(0, 100%, ${100 - 10 * pixel[i]}%)`;
            ctx.fillRect(i * 10, 0, 10, 10);
        }
    }

    display.next();
}, 50);
