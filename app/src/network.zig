const std = @import("std");
const mg = @cImport({
    @cInclude("mongoose.h");
});

const ws = "ws://localhost:8000/websocket";

pub var transport = Network{ .position = -1 };

pub const Network = struct {
    position: i32,

    pub fn set_position(pos: i32) void {
        transport.position = pos;
    }

    pub fn get_position() i32 {
        const value = transport.position;
        transport.position = -1;
        return value;
    }
};

fn event_handler(
    c: ?*mg.mg_connection,
    event: c_int,
    event_data: ?*anyopaque,
) callconv(.C) void {
    if (event == mg.MG_EV_WS_OPEN) {
        _ = mg.mg_ws_send(c, "connected", 5, mg.WEBSOCKET_OP_TEXT);
    }

    _ = event_data;
}

pub fn init() void {
    var event_mgr = mg.mg_mgr{};
    var is_done = false;

    mg.mg_mgr_init(&event_mgr);
    const conn = mg.mg_ws_connect(&event_mgr, ws, event_handler, &is_done, null);

    while (conn != null and !is_done) mg.mg_mgr_poll(&event_mgr, 1000);
}
