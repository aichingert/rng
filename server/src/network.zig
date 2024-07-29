const std = @import("std");
const packets = @import("packets");

const mg = @cImport({
    @cInclude("mongoose.h");
});

const server = "ws://localhost:8000";
var network = Network{
    .event_mgr = mg.mg_mgr{},
};

const MgConnection = extern struct {
    next: ?*MgConnection,
    mgr: ?*mg.mg_mgr,
    loc: mg.mg_addr,
    rem: mg.mg_addr,
    fd: ?*anyopaque,
    id: u32,
    recv: mg.mg_iobuf,
    send: mg.mg_iobuf,
    prof: mg.mg_iobuf,
    rtls: mg.mg_iobuf,
    fun: mg.mg_event_handler_t,
    fun_data: ?*anyopaque,
    pfn: mg.mg_event_handler_t,
    pfn_data: ?*anyopaque,
    data: [mg.MG_DATA_SIZE]u8,
    tls: ?*anyopaque,
    is_listening: bool,
    is_client: bool,
    is_accepted: bool,
    is_resolving: bool,
    is_arplooking: bool,
    is_connecting: bool,
    is_tls: bool,
    is_tls_hs: bool,
    is_udp: bool,
    is_websocket: bool,
    is_mqtt5: bool,
    is_hexdumping: bool,
    is_draining: bool,
    is_closing: bool,
    is_full: bool,
    is_resp: bool,
    is_readable: bool,
    is_writable: bool,
};

pub const Network = struct {
    event_mgr: mg.mg_mgr,

    const Self = @This();

    fn broadcast(self: *Self, packet: packets.Set) void {
        var next = @as(?*MgConnection, @ptrCast(@alignCast(self.event_mgr.conns)));
        const str: []const u8 = packet.encode();

        std.debug.print("broadcast: \n", .{});
        while (next) |conn| {
            std.debug.print("{?}\n", .{conn.is_listening});
            const c = @as(?*mg.mg_connection, @ptrCast(@alignCast(conn)));
            _ = mg.mg_ws_send(c, str.ptr, str.len, mg.WEBSOCKET_OP_TEXT);
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
            std.debug.print("{s}\n", .{wm.data.buf[5..]});
            const idx = std.fmt.parseInt(i32, std.mem.span(wm.data.buf[5..]), 10) catch |e| {
                std.debug.print("{?}\n", .{e});
                return;
            };

            network.broadcast(packets.Set{ .idx = idx });
        }
    }

    pub fn init() void {
        mg.mg_mgr_init(&network.event_mgr);
        _ = mg.mg_http_listen(&network.event_mgr, server, event_handler, null);

        //_ = network.event_mgr.conns.?.next;

        std.debug.print("WS listening on {s}\n", .{server});
        while (true) mg.mg_mgr_poll(&network.event_mgr, 100);
        mg.mgr_free(&network.event_mgr);
    }
};
