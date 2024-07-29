const std = @import("std");
const rl = @import("raylib");
const packets = @import("packets");
const Thread = std.Thread;
const Mutex = Thread.Mutex;

const mg = @cImport({
    @cInclude("mongoose.h");
});

const server = "ws://pattern.nitoa.at/websocket";

pub var tasks = Tasks.init();
pub var colors: [81]rl.Color = [_]rl.Color{rl.Color.light_gray} ** 81;

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
    // TODO: check that ws connection was successful if (event == mg.MG_EV_WS_OPEN) {

    if (event == mg.MG_EV_WS_MSG) {
        if (event_data) |data| {
            const wm = @as(*mg.mg_ws_message, @ptrCast(@alignCast(data)));

            if (std.fmt.parseInt(usize, std.mem.span(wm.data.buf[5..]), 10)) |idx| {
                colors[idx] = rl.Color.gray;
            } else |err| {
                std.debug.print("{?}\n", .{err});
            }
        }
    }

    const position = tasks.getPosition();

    if (position != -1) {
        const str = (packets.Set{ .idx = position }).encode();

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
