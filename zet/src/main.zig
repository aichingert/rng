const std = @import("std");
const options = @import("options");

const Platform = options.platform;

pub fn main() !void {
    if (Platform == .unix) {
        std.debug.print("well hello friends from unix\n", .{});
    }

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());

    try std.testing.expectEqual(Platform, "unix");
}
