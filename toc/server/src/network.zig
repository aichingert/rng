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
    .turn = true,
    .game = game.Game.new(), // FIXME: replace new with init in zig style
};

pub const Network = struct {
    event_mgr: mg.mg_mgr,

    const Self = @This();

    // FIXME: better understand connection to not have to filter them
    // like this as this is the most inefficent and dumbest method I
    // could think of
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
                    const set = packets.Set.decode(data);
                    const omv = lobby.player_one.?.conn_id == Connection.castFromCType(c).?.id;

                    //if (lobby.player_one.?.conn_id == lobby.player_two.?.conn_id) {
                    //    network.broadcast(packets.Set.encode(set.idx, 1, allocator), lobby.player_one.?.conn_id);
                    //    return;
                    //}

                    if (lobby.turn != omv) {
                        return;
                    }

                    if (lobby.turn and lobby.game.set(set.idx, 1)) {
                        if (lobby.game.won(1)) {
                            network.broadcast(packets.GameFinished.encode(1, allocator), lobby.player_one.?.conn_id);
                            network.broadcast(packets.GameFinished.encode(0, allocator), lobby.player_two.?.conn_id);

                            lobby.player_one = null;
                            lobby.player_two = null;
                            lobby.turn = true;
                            lobby.game = game.Game.new();

                            return;
                        }
                        network.broadcast(packets.Set.encode(set.idx, 1, allocator), lobby.player_one.?.conn_id);
                        network.broadcast(packets.Set.encode(set.idx, 0, allocator), lobby.player_two.?.conn_id);
                        lobby.turn = !lobby.turn;
                    } else if (lobby.game.set(set.idx, 2)) {
                        if (lobby.game.won(2)) {
                            network.broadcast(packets.GameFinished.encode(0, allocator), lobby.player_one.?.conn_id);
                            network.broadcast(packets.GameFinished.encode(1, allocator), lobby.player_two.?.conn_id);

                            lobby.player_one = null;
                            lobby.player_two = null;
                            lobby.turn = true;
                            lobby.game = game.Game.new();

                            return;
                        }

                        network.broadcast(packets.Set.encode(set.idx, 0, allocator), lobby.player_one.?.conn_id);
                        network.broadcast(packets.Set.encode(set.idx, 1, allocator), lobby.player_two.?.conn_id);
                        lobby.turn = !lobby.turn;
                    }
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

                    if (lobby.player_one == null or lobby.player_one.?.conn_id == conn_id) {
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
                .game_finished => {},
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
