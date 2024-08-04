const std = @import("std");
const rl = @import("raylib");
const packets = @import("packets");

const mg = @cImport({
    @cInclude("mongoose.h");
});

pub const allocator = std.heap.page_allocator;

const Thread = std.Thread;
const Mutex = Thread.Mutex;

const server = "ws://pattern.nitoa.at/websocket";
//const server = "ws://localhost:8000/websocket";

pub const GameScreen = enum {
    login,
    lobby,
    outcome,
    game,
};

pub var tasks = Tasks.init();
pub var screen: GameScreen = GameScreen.login;
pub var enemy: []u8 = undefined;
pub var outcome: u1 = 0;

pub var colors: [81]rl.Color = [_]rl.Color{rl.Color.white} ** 81;
const players: [2]rl.Color = [_]rl.Color{ rl.Color.blue, rl.Color.orange };

pub const Tasks = struct {
    mutex: Mutex,
    packet: ?[]const u8,
    username: []u8,

    const Self = @This();

    pub fn init() Self {
        return Self{
            .mutex = Mutex{},
            .packet = null,
            .username = undefined,
        };
    }

    pub fn blockForLogin(self: *Self) void {
        self.mutex.lock();
    }

    pub fn unblockForLogin(self: *Self, name: []const u8) void {
        self.username = allocator.alloc(u8, name.len) catch {
            @panic("it's over");
        };
        @memcpy(self.username, name);
        self.mutex.unlock();
    }

    pub fn setPacket(self: *Self, data: []const u8) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.packet = data;
    }

    pub fn getPacket(self: *Self) ?[]const u8 {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.packet == null) {
            return null;
        }

        const data = allocator.alloc(u8, self.packet.?.len) catch {
            return null;
        };

        @memcpy(data, self.packet.?);
        self.packet = null;
        return data;
    }
};

fn event_handler(
    c: ?*mg.mg_connection,
    event: c_int,
    event_data: ?*anyopaque,
) callconv(.C) void {
    // TODO: check that ws connection was successful if (event == mg.MG_EV_WS_OPEN) {
    if (event == mg.MG_EV_WS_OPEN) {
        const str = packets.JoinLobbyServer.encode(tasks.username, allocator);
        _ = mg.mg_ws_send(c, str.ptr, str.len, mg.WEBSOCKET_OP_TEXT);
    }

    if (event == mg.MG_EV_WS_MSG) {
        if (event_data) |data| {
            const wm = @as(*mg.mg_ws_message, @ptrCast(@alignCast(data)));
            const buf: []const u8 = std.mem.span(wm.data.buf);

            switch (packets.PacketType.getType(buf)) {
                .game_set => {
                    const set = packets.Set.decode(buf);
                    colors[set.idx] = players[@as(usize, set.color)];
                },
                .game_enqueue => {
                    enemy = allocator.alloc(u8, buf.len - 1) catch {
                        @panic("no");
                    };

                    @memcpy(enemy, buf[1..]);
                    screen = GameScreen.game;
                },
                .game_finished => {
                    outcome = packets.GameFinished.decode(buf).outcome;
                    colors = [_]rl.Color{rl.Color.white} ** 81;
                    screen = GameScreen.outcome;
                },
                else => {
                    std.debug.print("{s}\n", .{buf});
                },
            }
        }
    }

    if (tasks.getPacket()) |packet| {
        _ = mg.mg_ws_send(c, packet.ptr, packet.len, mg.WEBSOCKET_OP_TEXT);
    }
}

pub fn init() void {
    var event_mgr = mg.mg_mgr{};
    mg.mg_mgr_init(&event_mgr);
    var is_done = false;

    // Waiting for the mutex so the username is set
    _ = tasks.getPacket();

    const conn = mg.mg_ws_connect(&event_mgr, server, event_handler, &is_done, null);

    while (conn != null and !is_done) mg.mg_mgr_poll(&event_mgr, 100);
}
