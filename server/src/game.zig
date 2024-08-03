const std = @import("std");
const Allocator = std.mem.Allocator;

const mg = @cImport({
    @cInclude("mongoose.h");
});

const board_size = 3;
const size = board_size * board_size;

pub const Lobby = struct {
    player_one: ?*Player,
    player_two: ?*Player,

    game: Game,
};

pub const Player = struct {
    name: []const u8,
    conn_id: u32,

    const Self = @This();

    pub fn new(name: []const u8, conn_id: u32, a: Allocator) ?*Self {
        var player = a.create(Self) catch {
            return null;
        };

        player.name = name;
        player.conn_id = conn_id;
        return player;
    }
};

pub const Game = struct {
    board: u128,

    pub fn new() Game {
        return Game{ .board = 0 };
    }

    pub fn set(self: *Game, z: i32, y: i32, x: i32) bool {
        std.debug.print("{} \n", .{z * size + y * board_size + x});

        _ = self;
        return false;
    }
};
