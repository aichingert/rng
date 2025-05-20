importScripts("./pkg/client.js");

async function worker_init() {
    // loops
    self.postMessage(0);
    await wasm_bindgen("./pkg/client_bg.wasm");
}

worker_init();
