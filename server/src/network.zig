const std = @import("std");
const packets = @import("packets");
const Connection = @import("connection.zig").Connection;

const mg = @cImport({
    @cInclude("mongoose.h");
});

const allocator = std.heap.page_allocator;
const server = "ws://localhost:8000";
var network = Network{
    .event_mgr = mg.mg_mgr{},
};

// TODO: running games for spectating

var waiting = std.AutoArrayHashMap(u32, ?*mg.mg_connection).init(allocator);

pub const Network = struct {
    event_mgr: mg.mg_mgr,

    const Self = @This();

    fn broadcast(self: *Self, packet: packets.Set) void {
        var next = Connection.castFromCType(self.event_mgr.conns);
        const str: []const u8 = packet.encode();

        while (next) |conn| {
            _ = mg.mg_ws_send(conn.castToCType(), str.ptr, str.len, mg.WEBSOCKET_OP_TEXT);
            next = conn.next;
        }

        std.debug.print("finished\n", .{});
    }

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
            const data: []const u8 = std.mem.span(wm.data.buf);

            switch (packets.PacketType.getType(data)) {
                .join_lobby => {},
                else => {},
            }

            //network.broadcast(packets.Set{ .idx = idx });
        }
    }

    pub fn init() void {
        mg.mg_mgr_init(&network.event_mgr);
        _ = mg.mg_http_listen(&network.event_mgr, server, event_handler, null);

        std.debug.print("WS listening on {s}\n", .{server});

        while (true) mg.mg_mgr_poll(&network.event_mgr, 100);
        mg.mgr_free(&network.event_mgr);
    }
};
