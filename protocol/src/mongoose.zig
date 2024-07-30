// Mongoose wrapper

const std = @import("std");
const mg = @cImport({
    @cInclude("mongoose.h");
});

pub const Manager = mg.mg_mgr;
pub const WsMessage = mg.mg_ws_message;
pub const HttpMessage = mg.mg_http_message;

pub fn mgrInit(mg_mgr: *Manager) void {
    mg.mg_mgr_init(mg_mgr);
}

pub fn mgrFree(mg_mgr: *Manager) void {
    mg.mgr_free(mg_mgr);
}

pub fn mgrPoll(mg_mgr: *Manager) void {
    mg.mg_mgr_poll(mg_mgr, 100);
}

pub fn httpListen(
    mg_mgr: *Manager,
    address: []const u8,
    func: fn (?*MgConnection, c_int, ?*anyopaque) callconv(.C) void,
    func_data: ?*anyopaque,
) void {
    _ = mg_mgr;
    _ = address;
    _ = func;
    _ = func_data;
}

pub const Event = enum {
    pub const http_msg: c_int = mg.MG_EV_HTTP_MSG;
    pub const ws_msg: c_int = mg.MG_EV_WS_MSG;
};

pub const MgConnection = mg.mg_connection;

pub fn toConnection(conn: ?*MgConnection) ?*Connection {
    return @as(?*Connection, @ptrCast(@alignCast(conn)));
}

pub const Connection = extern struct {
    next: ?*Connection,
    mgr: ?*mg.mg_mgr,
    loc: mg.mg_addr,
    rem: mg.mg_addr,
    fd: ?*anyopaque,
    id: u32,
    recv: mg.mg_iobuf,
    send: mg.mg_iobuf,
    prof: mg.mg_iobuf,
    rtls: mg.mg_iobuf,
    fun: mg.mg_event_handler_t,
    fun_data: ?*anyopaque,
    pfn: mg.mg_event_handler_t,
    pfn_data: ?*anyopaque,
    data: [mg.MG_DATA_SIZE]u8,
    tls: ?*anyopaque,
    is_listening: bool,
    is_client: bool,
    is_accepted: bool,
    is_resolving: bool,
    is_arplooking: bool,
    is_connecting: bool,
    is_tls: bool,
    is_tls_hs: bool,
    is_udp: bool,
    is_websocket: bool,
    is_mqtt5: bool,
    is_hexdumping: bool,
    is_draining: bool,
    is_closing: bool,
    is_full: bool,
    is_resp: bool,
    is_readable: bool,
    is_writable: bool,

    const Self = @This();

    pub fn toMgConnection(self: ?*Self) ?*MgConnection {
        return @as(?*MgConnection, @ptrCast(@alignCast(self)));
    }

    pub fn toWs(mgr: ?*Manager, buf: []const u8, func: fn (?*MgConnection, c_int, ?*anyopaque) callconv(.C) void, func_data: ?*anyopaque, fmt: [*c]u8) ?*Self {
        const conn = mg.mg_ws_connect(mgr, buf.ptr, func, func_data, fmt);
        return @as(?*Connection, @ptrCast(@alignCast(conn)));
    }

    pub fn wsSend(self: *Self, buf: ?*const anyopaque, len: usize, op: i32) void {
        _ = mg.mg_ws_send(toMgConnection(self), buf, len, @as(c_int, op));
    }
};
