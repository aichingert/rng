const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "app",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const raylib_dep = b.dependency("raylib-zig", .{
        .target = target,
        .optimize = optimize,
    });
    const raylib = raylib_dep.module("raylib");
    const raylib_artifact = raylib_dep.artifact("raylib");

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

    exe.linkLibrary(lib);
    exe.linkLibrary(raylib_artifact);
    exe.root_module.addImport("raylib", raylib);

    const libprotocol = b.dependency("protocol", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("packets", libprotocol.module("packets"));

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
