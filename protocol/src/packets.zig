const std = @import("std");

pub const Set = struct {
    idx: i32,

    const Self = @This();

    pub fn decode(data: ?*anyopaque) ?*Self {
        const d = data orelse return null;
        return @as(*Self, @ptrCast(@alignCast(d)));
    }

    pub fn encode(self: *const Self) []const u8 {
        var buf: [20]u8 = undefined;
        const str: []const u8 = std.fmt.bufPrint(&buf, "set: {}", .{self.idx}) catch {
            return "none";
        };

        std.debug.print("{s}\n", .{str});
        return str;
    }
};
