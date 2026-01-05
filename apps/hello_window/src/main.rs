#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use models::*;
use thing_codec::GraphClient;
// Thing trait is exported by models

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("Hello Window Starting...");

    let mut client = SyscallGraphClient;

    // 1. Create Layout first
    let kind_layout = symbol_intern("kind.Layout");
    let layout_id = client.create_thing(kind_layout).expect("create layout");
    let layout = Layout {
        kind: LayoutKind::Column,
        padding: 12,
        gap: 10,
        align: 0, 
    };
    layout.write(&mut client, layout_id).expect("save layout");

    // 2. Create Window
    let kind_window = symbol_intern("kind.Window");
    let window_id = client.create_thing(kind_window).expect("create window");
    
    let window = Window {
        title: symbol_intern("Hello"),
        x: 100,
        y: 100,
        width: 480,
        height: 272,
        style: WindowStyle {
            bg_rgba: 0xF5F5F5FF, 
            radius: 10,
            shadow: 1, 
            elevation: 2,
        },
        content_root: layout_id,
    };
    window.write(&mut client, window_id).expect("save window");

    // 3. Create Widgets
    let kind_label = symbol_intern("kind.Label");
    let label_id = client.create_thing(kind_label).expect("create label");
    let label = Label {
        text: symbol_intern("Hello, ThingOS."),
        style: TextStyle { size: 16, color_rgba: 0x000000FF },
    };
    label.write(&mut client, label_id).expect("save label");

    let kind_button = symbol_intern("kind.Button");
    let button_id = client.create_thing(kind_button).expect("create button");
    let button = Button {
        text: symbol_intern("OK"),
        style: ButtonStyle { bg_rgba: 0xE0E0E0FF, radius: 4 },
    };
    button.write(&mut client, button_id).expect("save button");

    // 4. Links
    let rel_content = symbol_intern("content_root");
    relationship_create(rel_content, window_id, layout_id);

    let rel_child = symbol_intern("child");
    relationship_create(rel_child, layout_id, label_id);
    relationship_create(rel_child, layout_id, button_id);

    // 5. Publish to place.windows
    let place_windows = thing_find("place.windows").unwrap_or_else(|| {
        let pid = thing_create(symbol_intern("kind.Place"), ThingId::from_parts(0,0));
        thing_register_name(pid, "place.windows");
        pid
    });
    
    let rel_contains = symbol_intern("contains");
    relationship_create(rel_contains, place_windows, window_id);

    log_info("Hello Window Published!");

    // 6. Pulse Frame
    let kind_frame = symbol_intern("kind.Frame");
    let frame_id = client.create_thing(kind_frame).expect("create frame");
    let frame = Frame {
        window: window_id,
        seq: 1,
    };
    frame.write(&mut client, frame_id).expect("save frame");
    
    let rel_has_frame = symbol_intern("has_frame");
    relationship_create(rel_has_frame, window_id, frame_id);

    loop {
        thing_std::time::sleep_ms(1000);
    }
}

