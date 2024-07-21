pub const Game = struct {
    board: Board,
};

pub const Board = struct {
    pub const Tag = enum {
        leaf,
        root,
    };

    board: union(Tag) {
        leaf: [9]u16,
        root: [9]*const Board,
    },

    pub fn new(isLeaf: bool) Board {
        if (isLeaf) {
            return Board{ .board = .{ .leaf = [9]u16{ 0, 0, 0, 0, 0, 0, 0, 0, 0 } } };
        }

        return Board{ .board = .{ .root = [9]*const Board{
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
            &Board.new(true),
        } } };
    }
};
