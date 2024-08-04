const mg = @cImport({
    @cInclude("mongoose.h");
});

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

    pub fn castFromCType(c: ?*mg.mg_connection) ?*Self {
        return @as(?*Self, @ptrCast(@alignCast(c)));
    }

    pub fn castToCType(c: ?*Self) ?*mg.mg_connection {
        return @as(?*mg.mg_connection, @ptrCast(@alignCast(c)));
    }
};
