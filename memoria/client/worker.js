importScripts("./pkg/client.js");

const {Communicator} = wasm_bindgen;

async function worker_init() {
    await wasm_bindgen("./pkg/client_bg.wasm");

    let comm = await Communicator.new();

    while (true) {
        let res = await comm.next();
        if (res == null) break;
        await self.postMessage(res);
    }

    self.onerror = function(event) {
        console.error("ERROR: ", event);
    }
}

worker_init();
