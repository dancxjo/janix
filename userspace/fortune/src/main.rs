#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::time::Duration;

use abi::schema::{keys, kinds, rels};
use abi::ids::HandleId;
use llm::{ChatRequest, Message, Role, StreamingLlmClient};
use ollama::OllamaClient;
use stem::info;
use stem::petals::{AlignItems, Color, Flex, FontKey, JustifyContent, Scene, Styled, Text, Window};
use stem::thing::sys::{bytespace_read, create_node, describe_thing, find, link, prop_get, prop_set};
use stem::thing::ThingId;
use core::task::{RawWaker, RawWakerVTable, Waker};

struct OllamaConfig {
    server: String,
    model: String,
}

fn find_locale_conf() -> Option<ThingId> {
    let mut modules = [ThingId::default(); 128];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = describe_thing(modules[i], &mut buf).unwrap_or(0);
        let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");

        let mod_name = if let Some(pos) = desc.find("name: \"") {
            let rest = &desc[pos + 7..];
            if let Some(end) = rest.find('"') {
                &rest[..end]
            } else {
                continue;
            }
        } else {
            continue;
        };

        if mod_name == "locale.conf" || mod_name.ends_with("/locale.conf") {
            return Some(modules[i]);
        }
    }
    None
}

fn read_ollama_config() -> OllamaConfig {
    let mut server = String::from("https://forebrain.local:11434");
    let mut model = String::from("tinyllama");

    if let Some(mod_id) = find_locale_conf() {
        if let Ok(bs_id) = prop_get(mod_id, "bytespace") {
            let bs_thing = ThingId::from_u64(bs_id);
            let len = stem::thing::sys::bytespace_info(bs_thing).unwrap_or(0);
            if len > 0 {
                let mut buf = alloc::vec![0u8; len];
                if bytespace_read(bs_thing, 0, &mut buf).is_ok() {
                    if let Ok(content) = alloc::string::String::from_utf8(buf) {
                        for line in content.lines() {
                            if let Some(val) = line.strip_prefix("OLLAMA_SERVER=") {
                                server = val.trim().into();
                            } else if let Some(val) = line.strip_prefix("OLLAMA_MODEL=") {
                                model = val.trim().into();
                            }
                        }
                    }
                }
            }
        }
    }

    OllamaConfig { server, model }
}


#[stem::main]
fn main(_arg: usize) -> ! {
    info!("FORTUNE: Starting...");

    let mut window_id: Option<ThingId> = None;

    // Wait for UI Root
    let mut ui_crown = ThingId::default();
    while ui_crown.to_u64_lossy() == 0 {
        let mut ui_crowns = [ThingId::default(); 1];
        if let Ok(1) = find(kinds::UI_CROWN, &mut ui_crowns) {
            ui_crown = ui_crowns[0];
        } else {
            stem::sleep(Duration::from_millis(100));
        }
    }

    // Create Window
    let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
    link(win, rels::CHILD_OF, ui_crown).expect("link window");
    link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
    window_id = Some(win);

    prop_set(win, keys::UI_BG_COLOR, 0xFFFDF5E6).ok(); // OldLace

    // Window Layout
    prop_set(win, keys::UI_WIDTH, 400).ok();
    prop_set(win, keys::UI_HEIGHT, 300).ok();
    prop_set(win, keys::UI_X, 400).ok();
    prop_set(win, keys::UI_Y, 200).ok();

    let mut fortune_text = String::from("Seeking wisdom...");

    // Initial scene
    update_ui(win, &fortune_text);

    // Read Ollama configuration from locale.conf
    let config = read_ollama_config();
    info!("FORTUNE: Using Ollama server: {}, model: {}", config.server, config.model);

    // Initialize Ollama
    let client = OllamaClient::new(&config.server, &config.model);

    let req = ChatRequest {
        messages: vec![Message {
            role: Role::User,
            content: String::from("Tell me a short, wise fortune cookie message."),
        }],
        ..Default::default()
    };

    match client.chat_stream(req) {
        Ok(mut stream) => {
            fortune_text.clear();
            update_ui(win, &fortune_text);

            let waker = noop_waker();
            let mut cx = core::task::Context::from_waker(&waker);

            loop {
                match stream.poll_next(&mut cx) {
                    core::task::Poll::Ready(Ok(Some(delta))) => {
                        fortune_text.push_str(&delta.text);
                        update_ui(win, &fortune_text);
                        stem::sleep(Duration::from_millis(10));
                    }
                    core::task::Poll::Ready(Ok(None)) => break,
                    core::task::Poll::Ready(Err(e)) => {
                        fortune_text.push_str(&format!("\n[Error: {:?}]", e));
                        update_ui(win, &fortune_text);
                        break;
                    }
                    core::task::Poll::Pending => {
                        stem::sleep(Duration::from_millis(10));
                    }
                }
            }
        }
        Err(e) => {
            fortune_text = format!("Error connecting to Ollama: {:?}", e);
            update_ui(win, &fortune_text);
        }
    }

    loop {
        stem::sleep(Duration::from_secs(1));
    }
}

fn update_ui(win: ThingId, text: &str) {
    let scene = Scene::new().window(
        Window::new(win)
            .title("Fortune Cookie")
            .root(
                Flex::column()
                    .padding(20)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .push(
                        Text::new(text)
                            .font(FontKey::new("NotoSans-Regular").size(18))
                            .color(Color::rgb(0, 0, 0))
                    ),
            ),
    );
    stem::petals::publish_window(&scene).ok();
}

fn noop_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
}
