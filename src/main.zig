//! Minimal XCB + Cairo example in Zig
//! Dependencies: libc, xcb, cairo
//! libc and XCB should be installed by default. For cairo, install it with:
//! sudo apt-get install libcairo2-dev
//! build and run with:
//! zig build-exe xcb.zig -lc -lxcb -lcairo && ./xcb
const std = @import("std");
const c = @cImport({
    @cInclude("xcb/xcb.h");
    @cInclude("cairo/cairo-xcb.h");
    @cInclude("cairo/cairo.h");
});

/// Get the Id of the visual of a screen.
/// https://xcb.freedesktop.org/xlibtoxcbtranslationguide/
pub fn lookup_visual(s: *c.xcb_screen_t, visual: c.xcb_visualid_t) ?*c.xcb_visualtype_t {
    var depth_iter = c.xcb_screen_allowed_depths_iterator(s);
    // std.debug.print("TYPE INFO depth_iter: {}\n", .{@typeInfo(@TypeOf(depth_iter))});
    while (depth_iter.rem != 0) {
        var visual_iter = c.xcb_depth_visuals_iterator(depth_iter.data);
        while (visual_iter.rem != 0) {
            if (@ptrCast(*c.xcb_visualtype_t, visual_iter.data).visual_id == visual) {
                return visual_iter.data;
            }
            c.xcb_visualtype_next(&visual_iter);
        }
        c.xcb_depth_next(&depth_iter);
    }
    return null;
}

pub fn main() void {
    var display: ?[*]const u8 = null;
    var screen: ?[*]c_int = null;
    // std.debug.print("TYPE INFO screen: {}", .{@typeInfo(@TypeOf(screen))});
    const conn = c.xcb_connect(display, screen);
    // std.debug.print("TYPE INFO conn: {}\n", .{@typeInfo(@TypeOf(conn))});
    defer c.xcb_disconnect(conn);

    // we need to cast from unknown length pointer to single pointer because
    // otherwise the zig compiler complains about not supporting field access.
    const s = @ptrCast(*c.xcb_screen_t, c.xcb_setup_roots_iterator(c.xcb_get_setup(conn)).data);
    // std.debug.print("TYPE INFO s: {}\n", .{@typeInfo(@TypeOf(s))});
    // std.debug.print("TYPE INFO s.root {} s.root_visual {}\n", .{s.root, s.root_visual});

    const window_id = c.xcb_generate_id(conn);
    const x: i16 = 0;
    const y: i16 = 0;
    const width: u16 = 640;
    const height: u16 = 480;
    const border_width: u16 = 0;
    const win_class: u16 = @as(u16, c.XCB_WINDOW_CLASS_INPUT_OUTPUT);
    const depth = c.XCB_COPY_FROM_PARENT;
    const mask = 0;
    const values = null;

    _ = c.xcb_create_window(conn, depth, window_id, s.root, x, y, width, height, border_width, win_class, s.root_visual, mask, values);
    _ = c.xcb_map_window(conn, window_id);
    _ = c.xcb_flush(conn);

    const vis = lookup_visual(s, s.root_visual);
    var surf = c.cairo_xcb_surface_create(conn, window_id, vis, width, height);
    if (surf == null) return;

    var ctx = c.cairo_create(surf);
    _ = c.xcb_flush(conn);

    // render loop
    var toggle: bool = true;
    while (true) {
        if (toggle) {
            c.cairo_set_source_rgb(ctx, 1.0, 0.65, 0); // orange
        } else {
            c.cairo_set_source_rgb(ctx, 0, 0, 0); // black
        }
        toggle = !toggle;
        c.cairo_paint(ctx);
        c.cairo_surface_flush(surf);
        _ = c.xcb_flush(conn);
        std.time.sleep(1e9);
    }
}
