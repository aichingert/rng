const std = @import("std");

const board_size = 3;
const size = board_size * board_size;

pub const Game = struct {
    board: [size]u16,

    // 1 2 4
    // 81632

    pub fn new() Game {
        return Game{ .board = [_]u16{0} ** size };
    }

    pub fn set(self: *Game, z: i32, y: i32, x: i32) bool {
        std.debug.print("{} \n", .{z * size + y * board_size + x});

        _ = self;
        return false;
    }
};
