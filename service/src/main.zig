const std = @import("std");
const game = @import("game.zig");
const rpc = @cImport({
    @cInclude("rpc.h");
});

pub fn main() !void {
    const instance = game.Game{ .board = game.Board.new(false) };

    std.debug.print("{?}", .{instance});

    rpc.initServer("localhost:8123");
}
