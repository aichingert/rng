const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const packets_mod = b.addModule("packets", .{
        .root_source_file = b.path("src/packets.zig"),
    });
    const mongoose_mod = b.addModule("mongoose", .{
        .root_source_file = b.path("src/mongoose.zig"),
    });

    const mongoose = b.dependency("mongoose", .{
        .target = target,
        .optimize = optimize,
    });
    const lib = b.addStaticLibrary(.{
        .name = "mongoose",
        .target = target,
        .optimize = optimize,
    });

    if (target.result.os.tag == .windows) {
        lib.linkSystemLibrary("ws2_32");
    }

    lib.addIncludePath(mongoose.path("."));
    lib.addCSourceFiles(.{
        .root = .{ .dependency = .{
            .dependency = mongoose,
            .sub_path = "",
        } },
        .files = &.{"mongoose.c"},
        .flags = &.{},
    });
    lib.linkLibC();
    lib.installHeader(mongoose.path("mongoose.h"), "mongoose.h");
    mongoose_mod.linkLibrary(lib);

    const library_mod = b.addModule("protocol", .{ .root_source_file = b.path("protocol.zig"), .imports = &.{
        .{ .name = "packets", .module = packets_mod },
        .{ .name = "mongoose", .module = mongoose_mod },
    } });

    _ = library_mod;
}
