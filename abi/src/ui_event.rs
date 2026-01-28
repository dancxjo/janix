//! UI event wire format for Petals interactions.

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiEventKind {
    Clicked = 1,
    Toggled = 2,
}

impl UiEventKind {
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(UiEventKind::Clicked),
            2 => Some(UiEventKind::Toggled),
            _ => None,
        }
    }
}

pub const UI_EVENT_BYTES: usize = 24;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiEventWire {
    pub kind: u8,
    pub checked: u8,
    pub _pad: [u8; 2],
    pub node_id: u64,
    pub action_or_value: u64,
}

impl UiEventWire {
    pub fn new_clicked(node_id: u64, action_id: u64) -> Self {
        Self {
            kind: UiEventKind::Clicked as u8,
            checked: 0,
            _pad: [0; 2],
            node_id,
            action_or_value: action_id,
        }
    }

    pub fn new_toggled(node_id: u64, checked: bool, value_id: u64) -> Self {
        Self {
            kind: UiEventKind::Toggled as u8,
            checked: if checked { 1 } else { 0 },
            _pad: [0; 2],
            node_id,
            action_or_value: value_id,
        }
    }

    pub fn encode(&self, out: &mut [u8]) -> Option<usize> {
        if out.len() < UI_EVENT_BYTES {
            return None;
        }
        out[0] = self.kind;
        out[1] = self.checked;
        out[2] = 0;
        out[3] = 0;
        out[4..12].copy_from_slice(&self.node_id.to_le_bytes());
        out[12..20].copy_from_slice(&self.action_or_value.to_le_bytes());
        out[20..24].copy_from_slice(&0u32.to_le_bytes());
        Some(UI_EVENT_BYTES)
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < UI_EVENT_BYTES {
            return None;
        }
        let kind = bytes[0];
        let checked = bytes[1];
        let node_id = u64::from_le_bytes(bytes[4..12].try_into().ok()?);
        let action_or_value = u64::from_le_bytes(bytes[12..20].try_into().ok()?);
        Some(Self {
            kind,
            checked,
            _pad: [0; 2],
            node_id,
            action_or_value,
        })
    }
}
