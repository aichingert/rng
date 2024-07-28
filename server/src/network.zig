const std = @import("std");
const packets = @import("packets");

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

            const packet = packets.Set{ .idx = 10 };
            const str: []const u8 = packet.encode();

            var buf: [20]u8 = undefined;
            const hello: []const u8 = std.fmt.bufPrint(&buf, "set: {}", .{packet.idx}) catch {
                return;
            };

            std.debug.print("{?}({s}|{d}) - {?}({s}|{d})\n", .{ @TypeOf(str), str, str.len, @TypeOf(hello), hello, hello.len });
            _ = mg.mg_ws_send(c, str.ptr, str.len, mg.WEBSOCKET_OP_TEXT);
        }
    }

    pub fn init() void {
        var network = Network{
            .event_mgr = mg.mg_mgr{},
        };
        mg.mg_mgr_init(&network.event_mgr);
        _ = mg.mg_http_listen(&network.event_mgr, ws, event_handler, null);

        while (true) mg.mg_mgr_poll(&network.event_mgr, 100);
        mg.mgr_free(&network.event_mgr);
    }
};
