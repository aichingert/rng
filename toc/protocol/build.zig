const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const packets_mod = b.addModule("packets", .{
        .root_source_file = b.path("src/packets.zig"),
        .target = target,
        .optimize = optimize,
    });

    const library_mod = b.addModule("protocol", .{ .root_source_file = b.path("protocol.zig"), .imports = &.{
        .{ .name = "packets", .module = packets_mod },
    } });

    _ = library_mod;
}
