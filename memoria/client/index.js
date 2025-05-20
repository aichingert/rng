import init, { route, handle_location } from "./pkg/client.js";

async function run() {
    await init();

    window.onpopstate = handle_location;
    window.route = route;
}

run();



