const std = @import("std");
const packets = @import("packets");
const mg = @import("mongoose");

const server = "ws://localhost:8000";
var network = Network{
    .event_mgr = mg.Manager{},
};

pub const Network = struct {
    event_mgr: mg.Manager,

    const Self = @This();

    fn broadcast(self: *Self, packet: packets.Set) void {
        var next = mg.toConnection(self.event_mgr.conns);
        const str: []const u8 = packet.encode();

        std.debug.print("broadcast: \n", .{});
        while (next) |conn| {
            std.debug.print("{?}\n", .{conn.is_listening});

            // mg.WEBSOCKET_OP_TEXT = 1
            conn.wsSend(str.ptr, str.len, 1);
            next = conn.next;
        }
        std.debug.print("finished\n", .{});
    }

    fn event_handler(c: ?*mg.MgConnection, event: c_int, event_data: ?*anyopaque) callconv(.C) void {
        if (event == mg.Event.http_msg) {
            if (@as(?*mg.HttpMessage, @ptrCast(@alignCast(event_data)))) |hm| {
                if (mg.mg_match(hm.uri, mg.mg_str("/websocket"), null)) {
                    mg.mg_ws_upgrade(c, hm, null);
                }
            }
        }

        if (event != mg.Event.ws_msg) {
            return;
        }

        if (@as(?*mg.WsMessage, @ptrCast(@alignCast(event_data)))) |wm| {
            std.debug.print("{s}\n", .{wm.data.buf[5..]});
            const idx = std.fmt.parseInt(i32, std.mem.span(wm.data.buf[5..]), 10) catch |e| {
                std.debug.print("{?}\n", .{e});
                return;
            };

            network.broadcast(packets.Set{ .idx = idx });
        }
    }

    pub fn init() void {
        mg.mgrInit(&network.event_mgr);
        _ = mg.httpListen(&network.event_mgr, server, event_handler, null);

        std.debug.print("WS listening on {s}\n", .{server});

        while (true) mg.mgrPoll(&network.event_mgr);
        mg.mgrFree(&network.event_mgr);
    }
};
