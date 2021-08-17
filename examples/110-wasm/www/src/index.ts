import * as we from 'wasm_examples';

we.run();

const sayHelloBtn = document.getElementById('sayHello') as HTMLButtonElement;
const sayHelloWithMyAlertBtn = document.getElementById('sayHelloWithMyAlert') as HTMLButtonElement;
const workWithNumbersBtn = document.getElementById('workWithNumbers') as HTMLButtonElement;

// Define a global function that will be called by Rust
(<any>window).myAlert = (msg: string) => alert(msg);

// Call functions exported from Rust
sayHelloBtn.onclick = () => we.sayHello();
sayHelloWithMyAlertBtn.onclick = () => we.sayHelloWithMyAlert();

workWithNumbersBtn.onclick = () => {
    console.log(`The result is ${we.workWithNumbers(0x1ffffffffffffd, 1, 1).toString(16)}`);
    console.log(`The result is ${we.workWithNumbers(0x1ffffffffffffe, undefined, 1).toString(16)}`);
};
