const std = @import("std");

pub fn build(b: *std.Build) void {
    const decoder_mod = b.addModule("decoder", .{
        .root_source_file = b.path("src/decoder.zig"),
    });
    const packets_mod = b.addModule("packets", .{
        .root_source_file = b.path("src/packets.zig"),
    });

    const library_mod = b.addModule("protocol", .{ .root_source_file = b.path("protocol.zig"), .imports = &.{
        .{ .name = "decoder", .module = decoder_mod },
        .{ .name = "packets", .module = packets_mod },
    } });

    _ = library_mod;
}
