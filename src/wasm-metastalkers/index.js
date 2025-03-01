const fs = require('fs');
const wasmBuffer = fs.readFileSync('./factorial.wasm');
WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
const { factorial } = wasmModule.instance.exports;
console.log(factorial(5)); // Output: 120
});
