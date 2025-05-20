importScripts("./pkg/client.js");

const {Communicator} = wasm_bindgen;

async function worker_init() {
    await wasm_bindgen("./pkg/client_bg.wasm");

    var comm = Communicator.new();
    console.log("ini wokri");

    self.onmessage = async (event) => {
        console.log("ini loopri");
        // loops
        self.postMessage("10|20|30|40");

        console.log("sendi messagi");

        while (true) {
            var res = await comm.next(event.data);
            if (res == null) break;
            await self.postMessage(res);
        }
    }
    self.onerror = function(event) {
        console.error("ERROR: ", event);
    }
}

worker_init();
