use thing_models::{Thing, PropKey, PropValue, PropType};
use abi::ThingId;
use abi::syscall_defs::SymbolId;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct FramebufferInfo {
    pub id: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: SymbolId,
    pub buffer_count: u32,
    pub active_buffer: u32,
}

impl Thing for FramebufferInfo {
    const KIND: &'static str = "pkg.framebuffer.FramebufferInfo";
    const DESCRIPTION: &'static str = "Immutable facts about a framebuffer display";

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("width", PropType::U64),
            ("height", PropType::U64),
            ("stride", PropType::U64),
            ("pixel_format", PropType::Symbol),
            ("buffer_count", PropType::U64),
            ("active_buffer", PropType::U64),
        ]
    }

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("width".to_string(), PropValue::U64(self.width as u64)));
        out.push(("height".to_string(), PropValue::U64(self.height as u64)));
        out.push(("stride".to_string(), PropValue::U64(self.stride as u64)));
        out.push(("pixel_format".to_string(), PropValue::Symbol(self.pixel_format)));
        out.push(("buffer_count".to_string(), PropValue::U64(self.buffer_count as u64)));
        out.push(("active_buffer".to_string(), PropValue::U64(self.active_buffer as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = SymbolId(0);
        let mut buffer_count = 0;
        let mut active_buffer = 0;

        for prop_opt in props {
            if let Some((k, v)) = prop_opt {
                match (k.as_str(), v) {
                    ("width", PropValue::U64(val)) => width = *val as u32,
                    ("height", PropValue::U64(val)) => height = *val as u32,
                    ("stride", PropValue::U64(val)) => stride = *val as u32,
                    ("pixel_format", PropValue::Symbol(val)) => pixel_format = *val,
                    ("buffer_count", PropValue::U64(val)) => buffer_count = *val as u32,
                    ("active_buffer", PropValue::U64(val)) => active_buffer = *val as u32,
                    _ => {}
                }
            }
        }

        Self {
            id,
            width,
            height,
            stride,
            pixel_format,
            buffer_count,
            active_buffer,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: ThingId,
    pub index: u32,
    pub size_bytes: u64,
    pub shared_buffer_id: ThingId,
}

impl Thing for Buffer {
    const KIND: &'static str = "pkg.framebuffer.Buffer";
    const DESCRIPTION: &'static str = "A framebuffer buffer";

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("index", PropType::U64),
            ("size_bytes", PropType::U64),
            ("shared_buffer_id", PropType::U64), // ThingId stored as U64
        ]
    }

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("index".to_string(), PropValue::U64(self.index as u64)));
        out.push(("size_bytes".to_string(), PropValue::U64(self.size_bytes)));
        out.push(("shared_buffer_id".to_string(), PropValue::U64(self.shared_buffer_id.0)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut index = 0;
        let mut size_bytes = 0;
        let mut shared_buffer_id = ThingId(0);

        for prop_opt in props {
            if let Some((k, v)) = prop_opt {
                match (k.as_str(), v) {
                    ("index", PropValue::U64(val)) => index = *val as u32,
                    ("size_bytes", PropValue::U64(val)) => size_bytes = *val,
                    ("shared_buffer_id", PropValue::U64(val)) => shared_buffer_id = ThingId(*val),
                    _ => {}
                }
            }
        }

        Self {
            id,
            index,
            size_bytes,
            shared_buffer_id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum IntentState {
    Pending = 0,
    Done = 1,
    Error = 2,
}

#[derive(Debug, Clone)]
pub struct PresentIntent {
    pub id: ThingId,
    pub buffer_index: u32,
    pub state: IntentState,
    pub error_code: Option<u32>,
    pub error_message: Option<SymbolId>,
}

impl Thing for PresentIntent {
    const KIND: &'static str = "pkg.framebuffer.PresentIntent";
    const DESCRIPTION: &'static str = "Userland request to present a buffer";

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("buffer_index", PropType::U64),
            ("state", PropType::U64),
            ("error_code", PropType::U64),
            ("error_message", PropType::Symbol),
        ]
    }

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("buffer_index".to_string(), PropValue::U64(self.buffer_index as u64)));
        out.push(("state".to_string(), PropValue::U64(self.state as u32 as u64)));
        if let Some(ec) = self.error_code {
            out.push(("error_code".to_string(), PropValue::U64(ec as u64)));
        }
        if let Some(em) = self.error_message {
            out.push(("error_message".to_string(), PropValue::Symbol(em)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut buffer_index = 0;
        let mut state = IntentState::Pending;
        let mut error_code = None;
        let mut error_message = None;

        for prop_opt in props {
            if let Some((k, v)) = prop_opt {
                match (k.as_str(), v) {
                    ("buffer_index", PropValue::U64(val)) => buffer_index = *val as u32,
                    ("state", PropValue::U64(val)) => {
                        state = match *val {
                            1 => IntentState::Done,
                            2 => IntentState::Error,
                            _ => IntentState::Pending,
                        }
                    },
                    ("error_code", PropValue::U64(val)) => error_code = Some(*val as u32),
                    ("error_message", PropValue::Symbol(val)) => error_message = Some(*val),
                    _ => {}
                }
            }
        }

        Self {
            id,
            buffer_index,
            state,
            error_code,
            error_message,
        }
    }
}
