const std = @import("std");
const options = @import("options");

const Graphics = @import("Graphics.zig");

const glfw = @cImport({
    @cInclude("GLFW/glfw3.h");
});

pub fn main() !void {
    if (glfw.glfwInit() == 0) {
        std.debug.print("Oh no\n", .{});
    }

    const monitor = glfw.glfwGetPrimaryMonitor();
    const window = glfw.glfwCreateWindow(640, 480, "Ren", null, null);

    const mode = glfw.glfwGetVideoMode(monitor);

    glfw.glfwWindowHint(glfw.GLFW_RED_BITS, mode.*.redBits);
    glfw.glfwWindowHint(glfw.GLFW_GREEN_BITS, mode.*.greenBits);
    glfw.glfwWindowHint(glfw.GLFW_BLUE_BITS, mode.*.blueBits);

    //while (glfw.glfwWindowShouldClose(window) == 0) {}

    glfw.glfwDestroyWindow(window);
    glfw.glfwTerminate();
}

test "setup glfw" {
    try std.testing.expect(glfw.glfwInit() != 0);
    glfw.glfwTerminate();
}
