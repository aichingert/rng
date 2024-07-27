const std = @import("std");
const Thread = std.Thread;
const rl = @import("raylib");
const net = @import("network.zig");

const rect = 40;
const offset = 200;

pub fn main() anyerror!void {
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

    var colors: [81]rl.Color = [_]rl.Color{rl.Color.light_gray} ** 81;

    while (!rl.windowShouldClose()) {
        if (rl.isMouseButtonPressed(rl.MouseButton.mouse_button_left)) {
            const y = @divFloor(rl.getMouseY() - range, range);
            const x = @divFloor(rl.getMouseX() - offset, range);

            if (x >= 0 and x < 9 and y * 9 + x >= 0) {
                const dz = @divFloor(x, 3) + @divFloor(y, 3) * 3;
                const dx = @rem(x, 3);
                const dy = @rem(y, 3);

                //const idx = @as(usize, @intCast(y * 9 + x));
                const idx = @as(usize, @intCast(dz * 9 + dy * 3 + dx));
                std.debug.print("{} | {} {} => {}\n", .{ dz, dy, dx, idx });

                if (idx < 81) {
                    colors[idx] = rl.Color.gray;
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
                    colors[(y - 1) * 9 + x],
                );
            }
        }
    }
}
