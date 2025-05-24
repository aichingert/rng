importScripts("./pkg/client.js");

const {LobbyStream} = wasm_bindgen;

async function worker_init() {
    await wasm_bindgen("./pkg/client_bg.wasm");

    let stream = await LobbyStream.new();

    while (true) {
        let res = await stream.next();
        if (res == null) break;
        await self.postMessage(res);
    }
}

worker_init();
