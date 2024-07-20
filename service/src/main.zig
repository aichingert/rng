const std = @import("std");
const rpc = @cImport({
    @cInclude("rpc.h");
});

export fn add(a: u32, b: u32) u32 {
    return a + b;
}

pub fn main() !void {
    const address: [*c]const u8 = "localhost:8123";
    rpc.initServer(address);
}
