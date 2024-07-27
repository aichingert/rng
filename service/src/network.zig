const std = @import("std");
const mg = @cImport({
    @cInclude("mongoose.h");
});

const ws = "ws://localhost:8000";

pub const Network = struct {
    event_mgr: mg.mg_mgr,

    fn event_handler(c: ?*mg.mg_connection, event: c_int, event_data: ?*anyopaque) callconv(.C) void {
        if (event == mg.MG_EV_HTTP_MSG) {
            if (@as(?*mg.mg_http_message, @ptrCast(@alignCast(event_data)))) |hm| {
                if (mg.mg_match(hm.uri, mg.mg_str("/websocket"), null)) {
                    mg.mg_ws_upgrade(c, hm, null);
                }
            }
        }

        if (event != mg.MG_EV_WS_MSG) {
            return;
        }

        if (@as(?*mg.mg_ws_message, @ptrCast(@alignCast(event_data)))) |wm| {
            std.debug.print("{s}\n", .{wm.data.buf});
            _ = mg.mg_ws_send(c, wm.data.buf, wm.data.len, mg.WEBSOCKET_OP_TEXT);
        }
    }

    pub fn init() Network {
        var ev_mgr = mg.mg_mgr{};
        mg.mg_mgr_init(&ev_mgr);
        _ = mg.mg_http_listen(&ev_mgr, ws, event_handler, null);

        return Network{
            .event_mgr = ev_mgr,
        };
    }

    pub fn loop(self: *Network) void {
        while (true) mg.mg_mgr_poll(&self.event_mgr, 1000);
        mg.mgr_free(&self.event_mgr);
    }
};
