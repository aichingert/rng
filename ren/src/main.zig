const std = @import("std");
const options = @import("options");

const Graphics = @import("Graphics.zig");

const glfw = @cImport({
    @cDefine("GLFW_INCLUDE_VULKAN", {});
    @cInclude("GLFW/glfw3.h");
});

const vk = @import("vulkan");

fn errorCallback(error_code: c_int, description: [*c]const u8) callconv(.C) void {
    std.log.err("glfw: {}: {s}\n", .{ error_code, description });
}

test "setup glfw" {
    _ = glfw.glfwSetErrorCallback(errorCallback);
    try std.testing.expect(glfw.glfwInit() == glfw.GLFW_TRUE);
    defer glfw.glfwTerminate();

    try std.testing.expect(glfw.glfwVulkanSupported() == glfw.GLFW_TRUE);

    glfw.glfwWindowHint(glfw.GLFW_CLIENT_API, glfw.GLFW_NO_API);
    const window = glfw.glfwCreateWindow(640, 480, "ren", null, null);
    defer glfw.glfwDestroyWindow(window);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    //var counter: i32 = 0;

    //while (glfw.glfwWindowShouldClose(window) == 0) {
    //    counter += 1;

    //    std.debug.print("{d}\n", .{counter});
    //    if (counter > 20000000) {
    //        break;
    //    }
    //}

    _ = allocator;
}
