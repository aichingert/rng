const std = @import("std");
const options = @import("options");

const Graphics = @import("Graphics.zig");

pub fn main() !void {
    var graphics = Graphics.init();
    graphics.increment();
}
