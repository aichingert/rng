const std = @import("std");

const allocator = std.heap.page_allocator;

pub const Set = struct {
    idx: i32,

    const Self = @This();

    pub fn decode(data: ?*anyopaque) ?*Self {
        const d = data orelse return null;
        return @as(*Self, @ptrCast(@alignCast(d)));
    }

    pub fn encode(self: *const Self) []const u8 {
        const str: []const u8 = std.fmt.allocPrint(allocator, "set: {}", .{self.idx}) catch {
            return "none";
        };

        return str;
    }
};
