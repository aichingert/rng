const std = @import("std");
const game = @import("game.zig");
const mg = @cImport({
    @cInclude("mongoose.h");
});

pub fn main() !void {
    var instance = game.Game.new();
    _ = instance.set(5, 1, 2);

    var mgr = mg.mg_mgr{};
    mg.mg_mgr_init(&mgr);

    std.debug.print("{?} {?}\n", .{ instance, mgr });
}
