const std = @import("std");
const Thread = std.Thread;

const rl = @import("raylib");
const net = @import("network.zig");
const packets = @import("packets");

const rect = 40;
const offset = 200;

const screenWidth = 800;
const screenHeight = 600;

const pallete = [_]rl.Color{
    rl.Color.init(29, 32, 33, 255),
    rl.Color.init(249, 162, 69, 255),
    rl.Color.init(240, 122, 0, 255),
    rl.Color.init(212, 81, 1, 255),
};

const maxNameLen = 6;

const GameScreen = enum {
    login,
    lobby,
    game,
};

var frameCount: u32 = 0;

var input: [maxNameLen:0]u8 = undefined;
var index: usize = 0;
var screen = GameScreen.login;

pub fn main() anyerror!void {
    net.tasks.blockForLogin();
    const handle = try Thread.spawn(.{}, net.init, .{});
    defer handle.detach();

    const start = offset;
    const end = screenWidth - offset - rect;

    const range = (end - start) / 8;

    rl.initWindow(screenWidth, screenHeight, "toc");
    defer rl.closeWindow();
    rl.setTargetFPS(60);

    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        rl.clearBackground(pallete[0]);
        defer rl.endDrawing();

        switch (screen) {
            .game => {
                if (rl.isMouseButtonPressed(rl.MouseButton.mouse_button_left)) {
                    const y = @divFloor(rl.getMouseY() - range, range);
                    const x = @divFloor(rl.getMouseX() - offset, range);

                    if (x >= 0 and x < 9 and y * 9 + x >= 0) {
                        const idx = @as(usize, @intCast(y * 9 + x));

                        if (idx < 81) {
                            //net.tasks.setPosition(@as(i32, @intCast(idx)));
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
            .login => login(),
            .lobby => lobby(),
        }
    }
}

fn login() void {
    frameCount = (frameCount + 1) % 100_000_000 + 1;
    rl.setMouseCursor(@intFromEnum(rl.MouseCursor.mouse_cursor_ibeam));
    var key: c_int = rl.getCharPressed();

    if (rl.isKeyPressed(rl.KeyboardKey.key_backspace)) {
        if (index > 0) {
            index = index - 1;
            input[index] = 0;
        }
        return;
    }
    if (rl.isKeyPressed(rl.KeyboardKey.key_enter)) {
        net.tasks.unblockForLogin(@as([]const u8, &input));
        screen = GameScreen.lobby;
    }

    while (key > 0) {
        if ((key >= 32) and (key <= 126) and (index < maxNameLen)) {
            input[index] = @as(u8, @intCast(key));
            index = index + 1;
        }

        key = rl.getCharPressed();
    }

    const text = @as([*:0]const u8, @ptrCast(@alignCast(&input)));

    // Cursor blinking
    if (index < maxNameLen and ((@divFloor(frameCount, 20)) % 4) == 0) {
        rl.drawText(
            "_",
            screenWidth / 2 - 100 + 8 + rl.measureText(text, 40),
            180 + 12,
            40,
            rl.Color.orange,
        );
    }

    rl.drawText(text, screenWidth / 2 - 100 + 5, 180 + 8, 40, pallete[1]);
}

fn lobby() void {
    // screenWidth/2.0f - button.width/2.0f,
    // screenHeight/2.0f - button.height/NUM_FRAMES/2.0f,
    // (float)button.width, frameHeight
    const btn = rl.Rectangle.init(screenWidth / 2 - 250 / 2, screenHeight / 2 - 100, 250, 50);
    var color: rl.Color = pallete[0];

    // Check button state
    if (rl.checkCollisionPointRec(rl.getMousePosition(), btn)) {
        color = pallete[1];

        if (rl.isMouseButtonReleased(rl.MouseButton.mouse_button_left)) {
            std.debug.print("enqueue package\n", .{});
            net.tasks.setPacket(packets.GameEnqueue.encode(net.tasks.username, net.allocator));
        }
    }

    rl.drawRectangleLines(
        @intFromFloat(btn.x),
        @intFromFloat(btn.y),
        @intFromFloat(btn.width),
        @intFromFloat(btn.height),
        color,
    );

    rl.drawText(
        "enqueue",
        screenWidth / 2 - 80,
        screenHeight / 2 - 100,
        40,
        pallete[2],
    );
}
