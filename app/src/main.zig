const std = @import("std");
const Thread = std.Thread;
const rl = @import("raylib");
const net = @import("network.zig");

const rect = 40;
const offset = 200;

const background = rl.Color{ .r = 29, .b = 32, .g = 33, .a = 255 };

const maxNameLen = 6;

const GameScreen = enum {
    login,
    lobby,
    game,
};

pub fn main() anyerror!void {
    //const handle = try Thread.spawn(.{}, net.init, .{});
    //defer handle.detach();

    const screenWidth = 800;
    const screenHeight = 600;

    const start = offset;
    const end = screenWidth - offset - rect;

    const range = (end - start) / 8;

    rl.initWindow(screenWidth, screenHeight, "toc");
    defer rl.closeWindow();
    rl.setTargetFPS(60);

    //const textBox = rl.Rectangle{ };

    var input: [maxNameLen:0]u8 = undefined;
    var index: usize = 0;
    const screen = GameScreen.login;

    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        rl.clearBackground(background);
        defer rl.endDrawing();

        switch (screen) {
            .game => {
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
            },
            .login => {
                rl.setMouseCursor(@intFromEnum(rl.MouseCursor.mouse_cursor_ibeam));

                var key: c_int = rl.getCharPressed();

                while (key > 0) {
                    if ((key >= 32) and (key <= 125) and (index <= maxNameLen)) {
                        input[index] = @as(u8, @intCast(key));
                        index = index + 1;
                    }

                    key = rl.getCharPressed();
                }

                if (rl.isKeyPressed(rl.KeyboardKey.key_backspace)) {
                    if (index > 0) {
                        index = index - 1;
                    }
                }

                rl.drawText(@as([*:0]const u8, @ptrCast(@alignCast(&input))), screenWidth / 2 - 100 + 5, 180 + 8, 40, rl.Color.orange);
            },
            .lobby => {},
        }
    }
}
