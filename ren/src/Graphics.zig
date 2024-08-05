const options = @import("options");

id: u32,

const Self = @This();

const impl = switch (options.platform) {
    .web => @import("web/Gl.zig"),
    else => @panic("not yet supported"),
};

pub fn init() Self {
    return Self{
        .id = 10,
    };
}

pub fn increment(self: *Self) void {
    self.id += 1;
}
