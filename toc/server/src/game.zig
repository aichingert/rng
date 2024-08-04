const std = @import("std");
const Allocator = std.mem.Allocator;

const mg = @cImport({
    @cInclude("mongoose.h");
});

const board_size = 3;
const size = board_size * board_size;

pub const Lobby = struct {
    player_one: ?*Player,
    player_two: ?*Player,
    turn: bool,

    game: Game,
};

pub const Player = struct {
    name: []const u8,
    conn_id: u32,

    const Self = @This();

    pub fn new(name: []const u8, conn_id: u32, a: Allocator) ?*Self {
        var player = a.create(Self) catch {
            return null;
        };

        player.name = name;
        player.conn_id = conn_id;
        return player;
    }
};

pub const Game = struct {
    board: [9][10]u8,
    next: usize,

    const Self = @This();

    pub fn new() Self {
        return Self{
            .board = [_][10]u8{[_]u8{0} ** 10} ** 9,
            .next = 11,
        };
    }

    // ..|..
    // ..|..
    // -----
    // ..|..
    // ..|..

    pub fn won(self: *Self, p: u8) bool {
        // FIXME: do this in loops
        return self.board[0][9] == p and self.board[1][9] == p and self.board[2][9] == p or self.board[3][9] == p and self.board[4][9] == p and self.board[5][9] == p or self.board[6][9] == p and self.board[7][9] == p and self.board[8][9] == p or self.board[0][9] == p and self.board[4][9] == p and self.board[8][9] == p or self.board[2][9] == p and self.board[4][9] == p and self.board[6][9] == p or self.board[0][9] == p and self.board[3][9] == p and self.board[6][9] == p or self.board[1][9] == p and self.board[4][9] == p and self.board[7][9] == p or self.board[2][9] == p and self.board[5][9] == p and self.board[8][9] == p;
    }

    pub fn set(self: *Self, idx: usize, player: u8) bool {
        const y = idx / 9;
        const x = idx % 9;

        const w = 3 * @divFloor(y, 3) + @divFloor(x, 3);
        const z = 3 * (y % 3) + x % 3;

        if (self.board[w][9] != 0 or self.board[w][z] != 0 or self.next != w and self.next != 11) {
            return false;
        }

        self.board[w][z] = player;
        self.next = z;

        if (Game.checkForWin(self.board[w][0..9], player)) {
            self.board[w][9] = player;
        }

        if (self.board[z][9] != 0) {
            self.next = 11;
        }

        return true;
    }

    fn checkForWin(array: *[9]u8, p: u8) bool {
        return array[0] == p and array[1] == p and array[2] == p or array[3] == p and array[4] == p and array[5] == p or array[6] == p and array[7] == p and array[8] == p or array[0] == p and array[4] == p and array[8] == p or array[2] == p and array[4] == p and array[6] == p or array[0] == p and array[3] == p and array[6] == p or array[1] == p and array[4] == p and array[7] == p or array[2] == p and array[5] == p and array[8] == p;
    }
};
