//! UI event wire format for Petals interactions.

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiEventKind {
    Clicked = 1,
    Toggled = 2,
    Focus = 3,
    Blur = 4,
    TextInsert = 5,
    TextBackspace = 6,
    TextDelete = 7,
    CursorMove = 8,
    Submit = 9,
}

impl UiEventKind {
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(UiEventKind::Clicked),
            2 => Some(UiEventKind::Toggled),
            3 => Some(UiEventKind::Focus),
            4 => Some(UiEventKind::Blur),
            5 => Some(UiEventKind::TextInsert),
            6 => Some(UiEventKind::TextBackspace),
            7 => Some(UiEventKind::TextDelete),
            8 => Some(UiEventKind::CursorMove),
            9 => Some(UiEventKind::Submit),
            _ => None,
        }
    }
}

pub const UI_EVENT_TEXT_MAX: usize = 32;
pub const UI_EVENT_BYTES: usize = 64;
pub const UI_EVENT_FLAG_CHECKED: u8 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiEventWire {
    pub kind: u8,
    pub flags: u8,
    pub text_len: u8,
    pub _pad: u8,
    pub window_id: u64,
    pub target_id: u64,
    pub value: u64,
    pub delta: i32,
    pub text: [u8; UI_EVENT_TEXT_MAX],
}

impl UiEventWire {
    pub fn new_clicked(window_id: u64, target_id: u64, action_id: u64) -> Self {
        Self {
            kind: UiEventKind::Clicked as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: action_id,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_toggled(window_id: u64, target_id: u64, checked: bool, value_id: u64) -> Self {
        Self {
            kind: UiEventKind::Toggled as u8,
            flags: if checked { UI_EVENT_FLAG_CHECKED } else { 0 },
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: value_id,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_focus(window_id: u64, target_id: u64) -> Self {
        Self {
            kind: UiEventKind::Focus as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_blur(window_id: u64, target_id: u64) -> Self {
        Self {
            kind: UiEventKind::Blur as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_text_insert(window_id: u64, target_id: u64, text: &[u8]) -> Self {
        let mut event = Self {
            kind: UiEventKind::TextInsert as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        };
        let len = core::cmp::min(text.len(), UI_EVENT_TEXT_MAX);
        event.text[..len].copy_from_slice(&text[..len]);
        event.text_len = len as u8;
        event
    }

    pub fn new_text_backspace(window_id: u64, target_id: u64) -> Self {
        Self {
            kind: UiEventKind::TextBackspace as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_text_delete(window_id: u64, target_id: u64) -> Self {
        Self {
            kind: UiEventKind::TextDelete as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_cursor_move(window_id: u64, target_id: u64, delta: i32) -> Self {
        Self {
            kind: UiEventKind::CursorMove as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn new_submit(window_id: u64, target_id: u64) -> Self {
        Self {
            kind: UiEventKind::Submit as u8,
            flags: 0,
            text_len: 0,
            _pad: 0,
            window_id,
            target_id,
            value: 0,
            delta: 0,
            text: [0; UI_EVENT_TEXT_MAX],
        }
    }

    pub fn encode(&self, out: &mut [u8]) -> Option<usize> {
        if out.len() < UI_EVENT_BYTES {
            return None;
        }
        out[0] = self.kind;
        out[1] = self.flags;
        out[2] = self.text_len;
        out[3] = 0;
        out[4..12].copy_from_slice(&self.window_id.to_le_bytes());
        out[12..20].copy_from_slice(&self.target_id.to_le_bytes());
        out[20..28].copy_from_slice(&self.value.to_le_bytes());
        out[28..32].copy_from_slice(&self.delta.to_le_bytes());
        out[32..64].copy_from_slice(&self.text);
        Some(UI_EVENT_BYTES)
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < UI_EVENT_BYTES {
            return None;
        }
        let kind = bytes[0];
        let flags = bytes[1];
        let text_len = core::cmp::min(bytes[2] as usize, UI_EVENT_TEXT_MAX) as u8;
        let window_id = u64::from_le_bytes(bytes[4..12].try_into().ok()?);
        let target_id = u64::from_le_bytes(bytes[12..20].try_into().ok()?);
        let value = u64::from_le_bytes(bytes[20..28].try_into().ok()?);
        let delta = i32::from_le_bytes(bytes[28..32].try_into().ok()?);
        let mut text = [0u8; UI_EVENT_TEXT_MAX];
        text.copy_from_slice(&bytes[32..64]);
        Some(Self {
            kind,
            flags,
            text_len,
            _pad: 0,
            window_id,
            target_id,
            value,
            delta,
            text,
        })
    }

    pub fn checked(&self) -> bool {
        (self.flags & UI_EVENT_FLAG_CHECKED) != 0
    }

    pub fn text_bytes(&self) -> &[u8] {
        &self.text[..self.text_len as usize]
    }
}
