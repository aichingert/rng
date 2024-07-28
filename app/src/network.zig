const std = @import("std");
const Thread = std.Thread;
const Mutex = Thread.Mutex;

const mg = @cImport({
    @cInclude("mongoose.h");
});

const server = "ws://localhost:8000/websocket";

pub var tasks = Tasks.init();

pub const Tasks = struct {
    mutex: Mutex,
    position: i32,

    const Self = @This();

    pub fn init() Self {
        return Self{
            .position = -1,
            .mutex = Mutex{},
        };
    }

    pub fn setPosition(self: *Self, pos: i32) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        self.position = pos;
    }

    pub fn getPosition(self: *Self) i32 {
        self.mutex.lock();
        defer self.mutex.unlock();

        const value = self.position;
        self.position = -1;
        return value;
    }
};

fn event_handler(
    c: ?*mg.mg_connection,
    event: c_int,
    event_data: ?*anyopaque,
) callconv(.C) void {
    if (event == mg.MG_EV_WS_OPEN) {
        _ = mg.mg_ws_send(c, "connected", 9, mg.WEBSOCKET_OP_TEXT);
    }

    if (event == mg.MG_EV_WS_MSG) {
        if (event_data) |data| {
            const ws = @as(*mg.mg_ws_message, @ptrCast(@alignCast(data)));
            std.debug.print("{s}\n", .{ws.data.buf});
        }
    }

    const position = tasks.getPosition();

    if (position != -1) {
        var buf: [2]u8 = undefined;
        const str = std.fmt.bufPrint(&buf, "{}", .{position}) catch {
            return;
        };
        _ = mg.mg_ws_send(
            c,
            @as(?*const anyopaque, @ptrCast(str)),
            str.len,
            mg.WEBSOCKET_OP_TEXT,
        );
    }
}

pub fn init() void {
    var event_mgr = mg.mg_mgr{};
    var is_done = false;

    mg.mg_mgr_init(&event_mgr);
    const conn = mg.mg_ws_connect(&event_mgr, server, event_handler, &is_done, null);

    while (conn != null and !is_done) mg.mg_mgr_poll(&event_mgr, 100);
}
