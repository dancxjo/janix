#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std as std;
use abi::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::input::{Key, KeyEvent, ModState, KeyState};
use abi::wire::typed::{TypedBytes, TypeId, CodecId};
use models::core::input::{
    RawKeyEventStreamBody, KeyEventStreamBody, TextEventStreamBody
};
use models::builtins::ids::{
    THING_RAW_KEY_EVENT_STREAM_KIND,
    THING_TEXT_EVENT_STREAM_KIND,
    THING_KEY_EVENT_STREAM_KIND,
    THING_KEY_EVENT_STREAM_SCHEMA,
    THING_TEXT_EVENT_STREAM_SCHEMA,
};
use models::Thing;

mod normalization;
mod layout;

use normalization::normalize;
use layout::InputEngine;

struct InternalState {
    lshift: bool, rshift: bool,
    lctrl: bool, rctrl: bool,
    lalt: bool, ralt: bool,
    lwin: bool, rwin: bool,
    caps: bool, num: bool, scroll: bool,
}

impl InternalState {
    fn new() -> Self {
        Self {
            lshift: false, rshift: false,
            lctrl: false, rctrl: false,
            lalt: false, ralt: false,
            lwin: false, rwin: false,
            caps: false, num: false, scroll: false,
        }
    }

    fn update(&mut self, key: Key, state: KeyState) {
        let down = state == KeyState::Down;
        match key {
            Key::LShift => self.lshift = down,
            Key::RShift => self.rshift = down,
            Key::LCtrl => self.lctrl = down,
            Key::RCtrl => self.rctrl = down,
            Key::LAlt => self.lalt = down,
            Key::RAlt => self.ralt = down,
            Key::LWin => self.lwin = down,
            Key::RWin => self.rwin = down,
            Key::CapsLock => if down { self.caps = !self.caps; },
            Key::NumLock => if down { self.num = !self.num; },
            Key::ScrollLock => if down { self.scroll = !self.scroll; },
            _ => {}
        }
    }
    
    fn mods(&self) -> ModState {
        ModState {
            shift: self.lshift || self.rshift,
            ctrl: self.lctrl || self.rctrl,
            alt: self.lalt, 
            gui: self.lwin || self.rwin,
            caps_lock: self.caps,
            num_lock: self.num,
            scroll_lock: self.scroll,
            altgr: self.ralt,
        }
    }
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();
    std::debug::log("Input Service Starting...\n");

    let client = std::GraphClient::new();
    let mut input_engine = InputEngine::new();
    let mut internal_state = InternalState::new();
    
    // IDs (User-assigned for now)
    let raw_stream_id = ThingId(3001); 
    let key_stream_id = ThingId(3100);
    let text_stream_id = ThingId(3200);
    
    // Create/Publish KeyEventStream (3100)
    create_stream_body(&client, key_stream_id, THING_KEY_EVENT_STREAM_KIND, THING_KEY_EVENT_STREAM_SCHEMA, &KeyEventStreamBody {
        head_seq: 0,
        capacity: 64,
        dropped: 0,
        events: Vec::new(),
    });
    
    // Create/Publish TextEventStream (3200)
    create_stream_body(&client, text_stream_id, THING_TEXT_EVENT_STREAM_KIND, THING_TEXT_EVENT_STREAM_SCHEMA, &TextEventStreamBody {
        head_seq: 0,
        capacity: 64,
        dropped: 0,
        events: Vec::new(),
    });
    
    // Key Stream Body State
    let mut key_stream = KeyEventStreamBody {
        head_seq: 0, capacity: 64, dropped: 0, events: Vec::new(),
    };
    let mut text_stream = TextEventStreamBody {
        head_seq: 0, capacity: 64, dropped: 0, events: Vec::new(),
    };
    
    std::debug::log("Input Service: Published Streams. Entering Loop.\n");
    
    let mut last_processed_seq = 0u64;
    let mut buf = [0u8; 4096]; // Buffer for graph ops
    
    loop {
        // Poll Raw Stream
        let op = GraphOp::GetThing { id: raw_stream_id };
        if let Ok(GraphReply::Thing { bytes }) = client.call_op(&op, &mut buf) {
             if let Ok(thing) = postcard::from_bytes::<Thing>(&bytes) {
                  // Decode TypedBytes from ThingBody.bytes
                  if let Ok(typed) = postcard::from_bytes::<TypedBytes>(&thing.body.bytes) {
                      if let Ok(raw_body) = postcard::from_bytes::<RawKeyEventStreamBody>(&typed.bytes) {
                           let head = raw_body.head_seq;
                           if head > last_processed_seq {
                               // Calculate start index
                               let len = raw_body.events.len() as u64;
                               let start_seq_base = head.saturating_sub(len);
                               
                               for (i, raw_event) in raw_body.events.iter().enumerate() {
                                   let seq = start_seq_base + (i as u64) + 1;
                                   if seq <= last_processed_seq { continue; }
                                   
                                   // Process
                                   if let Some(key) = normalize(raw_event) {
                                        internal_state.update(key, raw_event.state);
                                        let mods = internal_state.mods();
                                        
                                        let key_event = KeyEvent {
                                            key,
                                            state: raw_event.state,
                                            mods,
                                            repeat: false,
                                            device: raw_event.source,
                                            time_ns: raw_event.time_ns,
                                        };
                                        
                                        // Key Stream
                                        key_stream.head_seq += 1;
                                        key_stream.events.push(key_event);
                                        if key_stream.events.len() > key_stream.capacity as usize {
                                            key_stream.events.remove(0);
                                            key_stream.dropped += 1;
                                        }
                                        
                                        // Layout
                                        if let Some(text_event) = input_engine.process(&key_event) {
                                            text_stream.head_seq += 1;
                                            text_stream.events.push(text_event);
                                            if text_stream.events.len() > text_stream.capacity as usize {
                                                text_stream.events.remove(0);
                                                text_stream.dropped += 1;
                                            }
                                        }
                                   }
                                   last_processed_seq = seq;
                               }
                               
                               // Publish
                               update_stream(&client, key_stream_id, THING_KEY_EVENT_STREAM_SCHEMA, &key_stream);
                               update_stream(&client, text_stream_id, THING_TEXT_EVENT_STREAM_SCHEMA, &text_stream);
                           }
                      }
                  }
             }
        }
        
        for _ in 0..10_000 { core::hint::spin_loop(); }
    }
}

fn create_stream_body<T: serde::Serialize>(client: &std::GraphClient, id: ThingId, kind: ThingId, schema: ThingId, body: &T) {
    let bytes = postcard::to_allocvec(body).expect("serialize");
    let typed = TypedBytes {
        type_id: TypeId(schema.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes,
    };
    let thing_body = models::ThingBody::from(&typed).expect("tb from typed");
    
    let mut out_buf = [0u8; 512];
    let _ = client.call_op(&GraphOp::UpdateThing { id, value: typed }, &mut out_buf);
}

fn update_stream<T: serde::Serialize>(client: &std::GraphClient, id: ThingId, schema: ThingId, body: &T) {
    let bytes = postcard::to_allocvec(body).expect("serialize");
    let typed = TypedBytes {
        type_id: TypeId(schema.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes,
    };
    let thing_body = models::ThingBody::from(&typed).expect("tb from typed");

    let mut out_buf = [0u8; 1024]; 
    let _ = client.call_op(&GraphOp::UpdateThing { id, value: typed }, &mut out_buf);
}
