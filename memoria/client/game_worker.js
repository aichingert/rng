importScripts("./pkg/client.js");

const {GameStream} = wasm_bindgen;

self.onmessage = async event => {
    await wasm_bindgen("./pkg/client_bg.wasm");

    let stream = await GameStream.new(event.data[0], event.data[1]);

    while (true) {
        let res = await stream.next();
        if (res == null) break;
        await self.postMessage(res);
    }
};
