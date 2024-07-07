const std = @import("std");
const cpp = @cImport({
    @cInclude("proto.h");
});

pub fn main() !void {
    cpp.helloWorld();
}
