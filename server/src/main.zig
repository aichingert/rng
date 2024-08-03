const std = @import("std");
const net = @import("network.zig");

pub fn main() !void {
    net.Network.init();
}
