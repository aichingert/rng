const std = @import("std");
const Thread = std.Thread;
const rl = @import("raylib");
const net = @import("network.zig");
const decoder = @import("decoder");

const rect = 40;
const offset = 200;

pub fn main() anyerror!void {
    decoder.hello();

    const handle = try Thread.spawn(.{}, net.init, .{});
    defer handle.detach();

    const screenWidth = 800;
    const screenHeight = 600;

    const start = offset;
    const end = screenWidth - offset - rect;

    const range = (end - start) / 8;

    rl.initWindow(screenWidth, screenHeight, "toc");
    defer rl.closeWindow();

    rl.setTargetFPS(60);

    while (!rl.windowShouldClose()) {
        if (rl.isMouseButtonPressed(rl.MouseButton.mouse_button_left)) {
            const y = @divFloor(rl.getMouseY() - range, range);
            const x = @divFloor(rl.getMouseX() - offset, range);

            if (x >= 0 and x < 9 and y * 9 + x >= 0) {
                const idx = @as(usize, @intCast(y * 9 + x));

                if (idx < 81) {
                    net.tasks.setPosition(@as(i32, @intCast(idx)));
                }
            }
        }

        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.white);

        for (0..9) |x| {
            const dx = @as(i32, @intCast(x));

            for (1..10) |y| {
                const dy = @as(i32, @intCast(y));

                rl.drawRectangle(
                    start + range * dx,
                    range * dy,
                    rect,
                    rect,
                    net.colors[(y - 1) * 9 + x],
                );
            }
        }
    }
}
