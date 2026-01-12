#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(any(feature = "alloc", feature = "std", test))]
extern crate alloc;

#[cfg(not(any(feature = "alloc", feature = "std", test)))]
compile_error!("bloom requires the `alloc` feature (or `std`) to build");

mod drawlist;
pub mod cursor;

pub use drawlist::{DrawCmd, DrawList};

/// Build a tiny demo drawlist with a centered rectangle.
pub fn demo_drawlist(screen_w: i32, screen_h: i32) -> DrawList {
    let mut list = DrawList::new();
    list.push(DrawCmd::Clear(0xFF101018));

    if screen_w <= 0 || screen_h <= 0 {
        return list;
    }

    let rect_w = (screen_w / 2).max(1);
    let rect_h = (screen_h / 2).max(1);
    let x = (screen_w - rect_w) / 2;
    let y = (screen_h - rect_h) / 2;

    list.push(DrawCmd::Rect {
        x,
        y,
        w: rect_w,
        h: rect_h,
        rgba: 0xFFFFCC00,
    });

    list
}
