const std = @import("std");
const ArrayList = std.ArrayList;

const allocator = std.heap.page_allocator;

pub const PacketType = enum {
    join_lobby,
    join_waiting,
    join_running,

    game_start,
    game_set,

    update_lobby,
    update_game,

    invalid,

    pub fn getType(data: []const u8) PacketType {
        if (data.len == 0 or data[0] < 48 or data[0] > 55) {
            return PacketType.invalid;
        }

        return @enumFromInt(data[0] - 48);
    }
};

pub const JoinLobby = struct {
    games: ArrayList(u16),

    const Self = @This();

    pub fn decode(data: []const u8) Self {
        var games = ArrayList(u16).init(allocator);

        var splits = std.mem.split(u8, data, " ");
        while (splits.next()) |chunk| {
            if (std.fmt.parseInt(u16, chunk, 10)) |game| {
                games.append(game);
            }
        }

        return JoinLobby{ .games = games };
    }

    pub fn encode(games: ArrayList(u16)) []const u8 {
        var str = "0";

        for (games) |game| {
            if (std.fmt.allocPrint(allocator, "{}", .{game})) |s| {
                str = str ++ " " ++ s;
            }
        }

        return str;
    }
};

pub const JoinWaiting = struct {};
pub const JoinPlaying = struct {};

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
