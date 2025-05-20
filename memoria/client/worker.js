importScripts("./pkg/client.js");

console.log("init worker");

async function worker_init() {
    // loops
    await wasm_bindgen("./pkg/client_bg.wasm");
}

worker_init();
