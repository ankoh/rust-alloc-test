const arrow_wasm = require("./pkg/rust_alloc");

const small = new arrow_wasm.alloc1(10);
console.log("small", small);

const large = new arrow_wasm.alloc2(1e9 / 4);
console.log("large", large);
console.log("small", small);
