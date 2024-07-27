const std = @import("std");

const board_size = 3;
const size = board_size * board_size;

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
