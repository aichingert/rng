// FIXME: proper error handling maybe just propogate
// the error instead of creating dummy data

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const PacketType = enum(u8) {
    join_lobby,
    join_waiting,
    join_running,

    game_set,
    game_enqueue,
    game_finished,

    update_lobby,

    invalid,

    pub fn getType(data: []const u8) PacketType {
        if (data.len == 0 or data[0] < 48 or data[0] > 55) {
            return PacketType.invalid;
        }

        return @enumFromInt(data[0] - 48);
    }
};

pub const GameEnqueue = struct {
    name: []const u8,

    const tag = @intFromEnum(PacketType.game_enqueue) + 48;
    const Self = @This();

    pub fn decode(data: []const u8) Self {
        return GameEnqueue{ .name = data[1..] };
    }

    pub fn encode(name: []const u8, a: Allocator) []const u8 {
        var str: []u8 = a.alloc(u8, name.len + 1) catch {
            // FIXME: error handling
            return "";
        };
        str[0] = tag;

        for (name, 1..) |c, i| {
            str[i] = c;
        }

        return str;
    }
};

pub const GameFinished = struct {
    outcome: u1,

    const tag = @intFromEnum(PacketType.game_finished);
    const Self = @This();

    pub fn decode(data: []const u8) Self {
        return GameFinished{ .outcome = @as(u1, @intCast(data[1] - 48)) };
    }

    pub fn encode(outcome: u1, a: Allocator) []const u8 {
        const str: []const u8 = std.fmt.allocPrint(a, "{}{}", .{ tag, outcome }) catch {
            @panic("enc set");
        };

        return str;
    }
};

pub const JoinLobbyClient = struct {
    games: ArrayList(u16),

    const Self = @This();

    pub fn decode(data: []const u8, a: Allocator) Self {
        var games = ArrayList(u16).init(a);

        var splits = std.mem.split(u8, data, " ");
        _ = splits.next();
        while (splits.next()) |chunk| {
            if (std.fmt.parseInt(u16, chunk, 10)) |game| {
                games.append(game);
            }
        }

        return JoinLobbyClient{ .games = games };
    }

    pub fn encode(games: ArrayList(u16), a: Allocator) []const u8 {
        var str = "0";

        for (games) |game| {
            if (std.fmt.allocPrint(a, "{s} {}", .{ str, game })) |s| {
                str = s;
            }
        }

        return str;
    }
};

pub const JoinLobbyServer = struct {
    name: []const u8,

    const Self = @This();

    pub fn decode(data: []const u8) Self {
        return Self{ .name = data[2..] };
    }

    pub fn encode(name: []const u8, a: Allocator) []const u8 {
        return std.fmt.allocPrint(a, "0 {s}", .{name}) catch {
            @panic("enc join");
        };
    }
};

pub const JoinWaiting = struct {};
pub const JoinPlaying = struct {};

pub const Set = struct {
    idx: usize,
    color: u1,

    const tag = @intFromEnum(PacketType.game_set);
    const Self = @This();

    pub fn decode(data: []const u8) Self {
        var splits = std.mem.split(u8, data[1..], " ");
        var set = Set{
            .idx = 0,
            .color = 0,
        };

        if (splits.next()) |chunk| {
            set.idx = std.fmt.parseInt(usize, chunk, 10) catch {
                @panic("TODO: set:idx");
            };
        }
        if (splits.next()) |chunk| {
            set.color = std.fmt.parseInt(u1, chunk, 10) catch {
                @panic("TODO: set:color");
            };
        }

        return set;
    }

    pub fn encode(idx: usize, color: u1, a: Allocator) []const u8 {
        const str: []const u8 = std.fmt.allocPrint(a, "{}{} {}", .{ tag, idx, color }) catch {
            @panic("enc set");
        };

        return str;
    }
};
