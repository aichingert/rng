const std = @import("std");
const net = @import("network.zig");
const game = @import("game.zig");

pub fn main() !void {
    var instance = game.Game.new();
    _ = instance.set(5, 1, 2);

    net.Network.init();
}
