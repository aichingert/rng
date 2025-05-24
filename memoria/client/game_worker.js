importScripts("./pkg/client.js");

const {GameStream} = wasm_bindgen;

async function worker_init() {
    await wasm_bindgen("./pkg/client_bg.wasm");

    self.onmessage = async (msg_event) => {
        let stream = await GameStream.new(msg_event.data[0], msg_event.data[1]);

        console.log(stream);

        while (true) {
            let res = await stream.next();
            if (res == null) break;
            await self.postMessage(res);
        }
    };

    
}

worker_init();
