const {route, handle_location} = wasm_bindgen;

async function run() {
    await wasm_bindgen();

    handle_location();
    window.route = route;
    window.onpopstate = handle_location;
}

run();



