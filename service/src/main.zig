const std = @import("std");
const game = @import("game.zig");
const rpc = @cImport({
    @cInclude("rpc.h");
});

pub fn main() !void {
    var instance = game.Game.new();
    _ = instance.set(5, 1, 2);

    std.debug.print("{?}\n", .{instance});

    rpc.initServer("localhost:8123");
}
