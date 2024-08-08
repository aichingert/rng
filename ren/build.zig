const std = @import("std");

pub const Platform = enum {
    web,
    linux,
    win32,
    macos,

    pub fn fromTarget(target: std.Target) Platform {
        if (target.cpu.arch == .wasm32) return .web;
        if (target.os.tag == .windows) return .win32;
        if (target.os.tag == .macos) return .macos;
        return .linux;
    }
};

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const module = b.addModule("ren", .{
        .root_source_file = b.path("src/main.zig"),
        .optimize = optimize,
        .target = target,
    });

    const platform = b.option(
        Platform,
        "platform",
        "ren platform to use",
    ) orelse Platform.fromTarget(target.result);

    const build_options = b.addOptions();
    build_options.addOption(Platform, "platform", platform);

    module.addImport("options", build_options.createModule());
    setupVulkan(b, module);

    switch (platform) {
        .linux, .win32, .macos => {
            setupGlfw(b, module, platform);
        },
        else => {},
    }

    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    var iter = module.import_table.iterator();
    while (iter.next()) |e| {
        unit_tests.root_module.addImport(e.key_ptr.*, e.value_ptr.*);
    }
    setupGlfw(b, &unit_tests.root_module, platform);

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}

pub fn setupVulkan(b: *std.Build, module: *std.Build.Module) void {
    const registry = b.dependency("vulkan_headers", .{}).path("registry/vk.xml");
    const vk_gen = b.dependency("vulkan_zig", .{}).artifact("vulkan-zig-generator");
    const vk_generate_cmd = b.addRunArtifact(vk_gen);

    vk_generate_cmd.addArg(registry.getPath(b));

    const vulkan_zig = b.addModule("vulkan-zig", .{
        .root_source_file = vk_generate_cmd.addOutputFileArg("vk.zig"),
        .target = module.resolved_target orelse b.host,
        .optimize = module.optimize.?,
    });

    module.addImport("vulkan", vulkan_zig);
}

pub fn setupGlfw(b: *std.Build, module: *std.Build.Module, platform: Platform) void {
    const target = module.resolved_target orelse b.host;

    const glfw = b.dependency("glfw", .{
        .target = target,
        .optimize = module.optimize.?,
    });

    const libGlfw = b.addStaticLibrary(.{
        .name = "glfw",
        .target = target,
        .optimize = module.optimize.?,
    });

    libGlfw.linkLibC();
    libGlfw.addIncludePath(glfw.path("include"));

    switch (platform) {
        .win32 => {
            libGlfw.addCSourceFiles(.{
                .root = .{
                    .dependency = .{
                        .dependency = glfw,
                        .sub_path = "src",
                    },
                },
                .files = &.{
                    "platform.c",
                    "monitor.c",
                    "init.c",
                    "vulkan.c",
                    "input.c",
                    "context.c",
                    "window.c",
                    "osmesa_context.c",
                    "egl_context.c",
                    "null_init.c",
                    "null_monitor.c",
                    "null_window.c",
                    "null_joystick.c",
                    "wgl_context.c",
                    "win32_thread.c",
                    "win32_init.c",
                    "win32_monitor.c",
                    "win32_time.c",
                    "win32_joystick.c",
                    "win32_window.c",
                    "win32_module.c",
                },
                .flags = &.{"-D_GLFW_WIN32"},
            });
        },
        .linux => {
            libGlfw.addCSourceFiles(.{
                .root = .{
                    .dependency = .{
                        .dependency = glfw,
                        .sub_path = "src",
                    },
                },
                .files = &.{
                    "platform.c",
                    "monitor.c",
                    "init.c",
                    "vulkan.c",
                    "input.c",
                    "context.c",
                    "window.c",
                    "osmesa_context.c",
                    "egl_context.c",
                    "null_init.c",
                    "null_monitor.c",
                    "null_window.c",
                    "null_joystick.c",
                    "posix_time.c",
                    "posix_thread.c",
                    "posix_module.c",
                    "egl_context.c",
                    "xkb_unicode.c",
                    "linux_joystick.c",
                    "posix_poll.c",

                    // FIXME: only supports X11 right now

                    "x11_init.c",
                    "x11_monitor.c",
                    "x11_window.c",
                    "glx_context.c",
                },
                .flags = &.{},
            });

            libGlfw.defineCMacro("_GLFW_X11", "1");
            libGlfw.linkSystemLibrary("X11");
        },
        .macos => @panic("not yet supported"),
        else => @panic("wasm has the browser"),
    }

    libGlfw.installHeader(glfw.path("include/GLFW/glfw3.h"), "GLFW/glfw3.h");

    module.linkLibrary(libGlfw);
}
