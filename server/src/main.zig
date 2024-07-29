const std = @import("std");
const net = @import("network.zig");
//const game = @import("game.zig");

pub fn main() !void {
    net.Network.init();
}
