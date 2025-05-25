importScripts("./pkg/client.js");

const {GameStream} = wasm_bindgen;

self.onmessage = async event => {
    await wasm_bindgen("./pkg/client_bg.wasm");

    console.log("connecting");
    let stream = await GameStream.new(event.data[0], event.data[1]);
    console.log("connected");

    while (true) {
        let res = await stream.next();
        if (res == null) break;
        await self.postMessage(res);
    }
};
