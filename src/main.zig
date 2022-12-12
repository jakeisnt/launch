const c = @cImport({
    @cInclude("SDL2/SDL.h");
    @cInclude("SDL2/SDL_ttf.h");
});

const std = @import("std");
const print = std.debug.print;
const assert = std.debug.assert;
const ArrayList = std.ArrayList;

pub fn sdl() !void {
    if (c.SDL_Init(c.SDL_INIT_VIDEO) != 0) {
        c.SDL_Log("Unable to initialize SDL: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    }
    defer c.SDL_Quit();

    const screen = c.SDL_CreateWindow("My Game Window", c.SDL_WINDOWPOS_UNDEFINED, c.SDL_WINDOWPOS_UNDEFINED, 300, 73, c.SDL_WINDOW_OPENGL) orelse
        {
        c.SDL_Log("Unable to create window: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };
    defer c.SDL_DestroyWindow(screen);

    const renderer = c.SDL_CreateRenderer(screen, -1, 0) orelse {
        c.SDL_Log("Unable to create renderer: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };
    defer c.SDL_DestroyRenderer(renderer);

    const zig_bmp = @embedFile("zig.bmp");
    const rw = c.SDL_RWFromConstMem(zig_bmp, zig_bmp.len) orelse {
        c.SDL_Log("Unable to get RWFromConstMem: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };
    defer assert(c.SDL_RWclose(rw) == 0);

    const zig_surface = c.SDL_LoadBMP_RW(rw, 0) orelse {
        c.SDL_Log("Unable to load bmp: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };
    defer c.SDL_FreeSurface(zig_surface);

    const zig_texture = c.SDL_CreateTextureFromSurface(renderer, zig_surface) orelse {
        c.SDL_Log("Unable to create texture from surface: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };
    defer c.SDL_DestroyTexture(zig_texture);

    c.SDL_StartTextInput();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    defer {
        const leaked = gpa.deinit();
        if (leaked) {
            print("We leaked memory : (", .{});
        }
        // if (leaked) expect(false) catch @panic("Leaked memory when allocating!");
    }

    var composition: []u8 = undefined;
    var text = ArrayList(u8).init(allocator);
    var selection_len: i32 = 0;
    var cursor: i32 = 0;

    try text.appendSlice("a");

    // initialize SDL2_ttf
    if (c.TTF_Init() != 0) {
        c.SDL_Log("Unable to initialize SDL2_ttf: %s", c.TTF_GetError());
        return error.SDLInitializationFailed;
    }
    defer c.TTF_Quit();

    //this opens a font style and sets a size
    var Sans: *c.TTF_Font = c.TTF_OpenFont("src/Sans.ttf", 24) orelse {
        c.SDL_Log("Unable to retrieve TTF FOnt: %s", c.SDL_GetError());
        return error.SDLInitializationFailed;
    };

    // this is the color in rgb format,
    // maxing out all would give you the color white,
    // and it will be your text's color
    var Color = c.SDL_Color{
        .r = 0xFF,
        .g = 0xFF,
        .b = 0xFF,
        .a = 0xFF,
    };

    var quit = false;
    while (!quit) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            switch (event.type) {
                c.SDL_QUIT => {
                    quit = true;
                },
                c.SDL_TEXTINPUT => {
                    print("received text input event: {c}\n", .{event.text.text});
                    try text.appendSlice(&event.text.text);
                    print("resulting text: {c}\n", .{text.items});
                },
                c.SDL_TEXTEDITING => {
                    print("received text editing event\n", .{});
                    composition = &event.edit.text;
                    cursor = event.edit.start;
                    selection_len = event.edit.length;
                },
                else => {},
            }
        }

        try text.append(0);

        var itemLen = text.items.len - 1;

        // print("\"", .{});
        // for (text.items) |elem| {
        //     print("{c}", .{elem});
        // }
        // print("\"\n", .{});

        // slice syntax: `:0` is termination with a number.
        // the slice syntax we use takes a slice up to len terminating with the number 0
        // currently a panic! index out of bounds.

        // I think this is because the arraylist doesn't have new memory allocated to hold the slice, which ends up being a copy?

        print("panic yet?\n", .{});

        var messageText: [:0]const u8 = text.items[0..itemLen :0];

        print("panic yet?\n", .{});

        // cast causes ptr to be null
        // @ptrCast([*c]const u8, messageText)
        var surfaceMessage: *c.SDL_Surface =
            c.TTF_RenderText_Solid(Sans, messageText.ptr, Color);

        print("panic yet?\n", .{});

        // now you can convert it into a texture
        var Message: *c.SDL_Texture = c.SDL_CreateTextureFromSurface(renderer, surfaceMessage) orelse {
            c.SDL_Log("Unable to create font texture from surface: %s", c.SDL_GetError());
            return error.SDLInitializationFailed;
        };

        // Emacs mode that lets me insert these into compiled binaries as hidden debug labels
        // like the stickers in common lisp
        // does zig have a way to do this such that it won't impact memory alignment?

        print("panic yet?\n", .{});

        _ = text.popOrNull().?;

        var Message_rect: c.SDL_Rect = undefined; //create a rect
        Message_rect.x = 0; //controls the rect's x coordinate
        Message_rect.y = 0; // controls the rect's y coordinte
        Message_rect.w = 100; // controls the width of the rect
        Message_rect.h = 100; // controls the height of the rect

        // (0,0) is on the top left of the window/screen,
        // think a rect as the text's box,
        // that way it would be very simple to understand

        // Now since it's a texture, you have to put RenderCopy
        // in your game loop area, the area where the whole code executes

        // you put the renderer's name first, the Message,
        // the crop size (you can ignore this if you don't want
        // to dabble with cropping), and the rect which is the size
        // and coordinate of your texture

        // Need to assign all non-void values to null
        _ = c.SDL_RenderCopy(renderer, Message, null, &Message_rect);

        // Don't forget to free your surface and texture
        defer c.SDL_FreeSurface(surfaceMessage);
        defer c.SDL_DestroyTexture(Message);

        _ = c.SDL_RenderClear(renderer);
        // We render the text to the screen here:
        _ = c.SDL_RenderCopy(renderer, Message, null, null);
        c.SDL_RenderPresent(renderer);

        c.SDL_Delay(17);

        // print("{x}\n", composition);
        // print("{x}\n", text);
        // print("{d}\n", .{selection_len});
    }
}

fn print_slice() void {
    // print("itemLen is currently {d}\n", .{itemLen});
    // if (itemLen > 0) {
    //     // sub1 so we keep that 0
    //     itemLen = itemLen - 1;
    // }
}

pub fn main() !void {
    try sdl();
    // var text = ArrayList(u8).init(allocator);
    // text.append(0);
    // const slice: [:0]const u8 = text.items[0..itemLen :0];
}
