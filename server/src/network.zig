const std = @import("std");
const game = @import("game.zig");
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

var lobby: game.Lobby = game.Lobby{
    .player_one = null,
    .player_two = null,
    .game = game.Game.new(),
};

pub const Network = struct {
    event_mgr: mg.mg_mgr,

    const Self = @This();

    fn broadcast(self: *Self, packet: []const u8, id: u32) void {
        var next = Connection.castFromCType(self.event_mgr.conns);

        while (next) |conn| {
            if (conn.id == id) {
                _ = mg.mg_ws_send(
                    conn.castToCType(),
                    packet.ptr,
                    packet.len,
                    mg.WEBSOCKET_OP_TEXT,
                );
            }
            next = conn.next;
        }

        std.debug.print("finished\n", .{});
    }

    fn event_handler(
        c: ?*mg.mg_connection,
        event: c_int,
        event_data: ?*anyopaque,
    ) callconv(.C) void {
        if (event == mg.MG_EV_HTTP_MSG) {
            const opt_hm = @as(
                ?*mg.mg_http_message,
                @ptrCast(@alignCast(event_data)),
            );

            if (opt_hm) |hm| {
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
                .game_set => {
                    std.debug.print("set \n", .{});
                },
                .game_enqueue => {
                    if (lobby.player_two != null) {
                        return;
                    }

                    const conn_id = Connection.castFromCType(c).?.*.id;
                    const name = allocator.alloc(u8, data.len - 1) catch {
                        return;
                    };
                    @memcpy(name, data[1..]);
                    const player = game.Player.new(name, conn_id, allocator);

                    if (lobby.player_one == null) {
                        lobby.player_one = player;
                        return;
                    }

                    lobby.player_two = player;

                    const one = packets.GameEnqueue.encode(lobby.player_two.?.name, allocator);
                    const two = packets.GameEnqueue.encode(lobby.player_one.?.name, allocator);

                    network.broadcast(one, lobby.player_one.?.conn_id);
                    network.broadcast(two, lobby.player_two.?.conn_id);
                },

                // TODO: implement proper multiplayer and spectator support
                // (probably/hopefully refactor everything in the process)
                .join_lobby => {},
                .join_waiting => {},
                .join_running => {},
                .update_lobby => {},
                .invalid => {
                    std.debug.print("INVALID: {s}\n", .{data});
                },
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
