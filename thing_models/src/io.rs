use abi::{PropKey, PropType, PropValue, Thing, ThingId};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

fn serialize_irqs(irq_lines: &[u8]) -> String {
    let mut result = String::new();
    for (idx, irq) in irq_lines.iter().enumerate() {
        if idx > 0 {
            result.push(',');
        }
        let part = format!("{}", irq);
        result.push_str(&part);
    }
    result
}

fn parse_irqs(value: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for token in value.split(',') {
        if token.is_empty() {
            continue;
        }
        if let Ok(num) = token.parse::<u8>() {
            result.push(num);
        }
    }
    result
}

#[derive(Clone, Debug)]
pub struct IoPortRegion {
    pub id: ThingId,
    pub name: String,
    pub base_port: u16,
    pub port_count: u16,
    pub irq_lines: Vec<u8>,
    pub claimed_by: Option<ThingId>,
}

impl IoPortRegion {
    pub fn seed_props(
        name: &str,
        base_port: u16,
        port_count: u16,
        irq_lines: &[u8],
    ) -> Vec<(PropKey, PropValue)> {
        let mut props = Vec::new();
        props.push(("name", PropValue::Str(String::from(name))));
        props.push(("base_port", PropValue::U64(base_port as u64)));
        props.push(("port_count", PropValue::U64(port_count as u64)));
        props.push(("irq_lines", PropValue::Str(serialize_irqs(irq_lines))));
        props
    }
}

impl Thing for IoPortRegion {
    const KIND: &'static str = "IoPortRegion";
    const DESCRIPTION: &'static str =
        "Hardware I/O port range that can be managed by userland drivers.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name", PropValue::Str(self.name.clone())));
        out.push(("base_port", PropValue::U64(self.base_port as u64)));
        out.push(("port_count", PropValue::U64(self.port_count as u64)));
        out.push(("irq_lines", PropValue::Str(serialize_irqs(&self.irq_lines))));
        if let Some(owner) = self.claimed_by {
            out.push(("claimed_by", PropValue::U64(owner.0)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut base_port = 0;
        let mut port_count = 0;
        let mut irq_lines = Vec::new();
        let mut claimed_by = None;

        for prop in props.iter().flatten() {
            match prop.0 {
                "name" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        name = v.clone();
                    }
                }
                "base_port" => {
                    if let PropValue::U64(v) = prop.1 {
                        base_port = v as u16;
                    }
                }
                "port_count" => {
                    if let PropValue::U64(v) = prop.1 {
                        port_count = v as u16;
                    }
                }
                "irq_lines" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        irq_lines = parse_irqs(v);
                    }
                }
                "claimed_by" => {
                    if let PropValue::U64(v) = prop.1 {
                        claimed_by = Some(ThingId(v));
                    }
                }
                _ => {}
            }
        }

        IoPortRegion {
            id,
            name,
            base_port,
            port_count,
            irq_lines,
            claimed_by,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::Str),
            ("base_port", PropType::U64),
            ("port_count", PropType::U64),
            ("irq_lines", PropType::Str),
            ("claimed_by", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct InterruptRequest {
    pub id: ThingId,
    pub irq_line: u8,
    pub enabled: bool,
    pub owner_process: Option<ThingId>,
}

impl Thing for InterruptRequest {
    const KIND: &'static str = abi::graph_kinds::KIND_INTERRUPT_REQUEST;
    const DESCRIPTION: &'static str = "Userland request to enable or disable an IRQ line.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((abi::graph_kinds::PROP_IRQ_LINE, PropValue::U64(self.irq_line as u64)));
        out.push((abi::graph_kinds::PROP_ENABLED, PropValue::Bool(self.enabled)));
        if let Some(owner) = self.owner_process {
            out.push(("owner_process", PropValue::U64(owner.0)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut irq_line = 0u8;
        let mut enabled = false;
        let mut owner_process = None;
        for (k, v) in props.iter().flatten() {
            match *k {
                abi::graph_kinds::PROP_IRQ_LINE => if let PropValue::U64(x) = v { irq_line = *x as u8; },
                abi::graph_kinds::PROP_ENABLED => if let PropValue::Bool(b) = v { enabled = *b; },
                "owner_process" => if let PropValue::U64(x) = v { owner_process = Some(ThingId(*x)); },
                _ => {}
            }
        }
        Self { id, irq_line, enabled, owner_process }
    }

    fn schema() -> &'static [(PropKey, PropType)] {
        &[
            (abi::graph_kinds::PROP_IRQ_LINE, PropType::U64),
            (abi::graph_kinds::PROP_ENABLED, PropType::Bool),
            ("owner_process", PropType::U64),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoDirection {
    Read,
    Write,
}

impl IoDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            IoDirection::Read => "Read",
            IoDirection::Write => "Write",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Read" => Some(IoDirection::Read),
            "Write" => Some(IoDirection::Write),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoWidth {
    U8,
    U16,
    U32,
}

impl IoWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            IoWidth::U8 => "U8",
            IoWidth::U16 => "U16",
            IoWidth::U32 => "U32",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "U8" => Some(IoWidth::U8),
            "U16" => Some(IoWidth::U16),
            "U32" => Some(IoWidth::U32),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl IoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            IoStatus::Pending => "Pending",
            IoStatus::InProgress => "InProgress",
            IoStatus::Completed => "Completed",
            IoStatus::Failed => "Failed",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Pending" => Some(IoStatus::Pending),
            "InProgress" => Some(IoStatus::InProgress),
            "Completed" => Some(IoStatus::Completed),
            "Failed" => Some(IoStatus::Failed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IoPortOp {
    pub id: ThingId,
    pub region_id: ThingId,
    pub offset: u16,
    pub direction: IoDirection,
    pub width: IoWidth,
    pub value: u32,
    pub status: IoStatus,
    pub error_code: u32,
    pub issued_by: ThingId,
}

impl IoPortOp {
    pub fn new(
        region_id: ThingId,
        offset: u16,
        direction: IoDirection,
        width: IoWidth,
        value: u32,
        issued_by: ThingId,
    ) -> Self {
        IoPortOp {
            id: ThingId(0),
            region_id,
            offset,
            direction,
            width,
            value,
            status: IoStatus::Pending,
            error_code: 0,
            issued_by,
        }
    }
}

impl Thing for IoPortOp {
    const KIND: &'static str = "IoPortOp";
    const DESCRIPTION: &'static str = "Request for a hardware I/O port read or write operation.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("region_id", PropValue::U64(self.region_id.0)));
        out.push(("offset", PropValue::U64(self.offset as u64)));
        out.push(("direction", PropValue::Str(self.direction.as_str().into())));
        out.push(("width", PropValue::Str(self.width.as_str().into())));
        out.push(("value", PropValue::U64(self.value as u64)));
        out.push(("status", PropValue::Str(self.status.as_str().into())));
        out.push(("error_code", PropValue::U64(self.error_code as u64)));
        out.push(("issued_by", PropValue::U64(self.issued_by.0)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut region_id = ThingId(0);
        let mut offset = 0;
        let mut direction = IoDirection::Read;
        let mut width = IoWidth::U8;
        let mut value = 0;
        let mut status = IoStatus::Pending;
        let mut error_code = 0;
        let mut issued_by = ThingId(0);

        for prop in props.iter().flatten() {
            match prop.0 {
                "region_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        region_id = ThingId(v);
                    }
                }
                "offset" => {
                    if let PropValue::U64(v) = prop.1 {
                        offset = v as u16;
                    }
                }
                "direction" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        if let Some(dir) = IoDirection::from_str(v) {
                            direction = dir;
                        }
                    }
                }
                "width" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        if let Some(w) = IoWidth::from_str(v) {
                            width = w;
                        }
                    }
                }
                "value" => {
                    if let PropValue::U64(v) = prop.1 {
                        value = v as u32;
                    }
                }
                "status" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        if let Some(s) = IoStatus::from_str(v) {
                            status = s;
                        }
                    }
                }
                "error_code" => {
                    if let PropValue::U64(v) = prop.1 {
                        error_code = v as u32;
                    }
                }
                "issued_by" => {
                    if let PropValue::U64(v) = prop.1 {
                        issued_by = ThingId(v);
                    }
                }
                _ => {}
            }
        }

        IoPortOp {
            id,
            region_id,
            offset,
            direction,
            width,
            value,
            status,
            error_code,
            issued_by,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("region_id", PropType::U64),
            ("offset", PropType::U64),
            ("direction", PropType::Str),
            ("width", PropType::Str),
            ("value", PropType::U64),
            ("status", PropType::Str),
            ("error_code", PropType::U64),
            ("issued_by", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct InterruptEvent {
    pub id: ThingId,
    pub irq_line: u8,
    pub region_id: Option<ThingId>,
    pub timestamp_ticks: u64,
}

impl InterruptEvent {
    pub fn props_for(
        irq_line: u8,
        region_id: Option<ThingId>,
        timestamp_ticks: u64,
    ) -> Vec<(PropKey, PropValue)> {
        let mut props = Vec::new();
        props.push(("irq_line", PropValue::U64(irq_line as u64)));
        if let Some(region) = region_id {
            props.push(("region_id", PropValue::U64(region.0)));
        }
        props.push(("timestamp_ticks", PropValue::U64(timestamp_ticks)));
        props
    }
}

impl Thing for InterruptEvent {
    const KIND: &'static str = "InterruptEvent";
    const DESCRIPTION: &'static str = "Kernel notification that an IRQ line fired.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("irq_line", PropValue::U64(self.irq_line as u64)));
        if let Some(region) = self.region_id {
            out.push(("region_id", PropValue::U64(region.0)));
        }
        out.push(("timestamp_ticks", PropValue::U64(self.timestamp_ticks)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut irq_line = 0;
        let mut region_id = None;
        let mut timestamp_ticks = 0;

        for prop in props.iter().flatten() {
            match prop.0 {
                "irq_line" => {
                    if let PropValue::U64(v) = prop.1 {
                        irq_line = v as u8;
                    }
                }
                "region_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        region_id = Some(ThingId(v));
                    }
                }
                "timestamp_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        timestamp_ticks = v;
                    }
                }
                _ => {}
            }
        }

        InterruptEvent {
            id,
            irq_line,
            region_id,
            timestamp_ticks,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("irq_line", PropType::U64),
            ("region_id", PropType::U64),
            ("timestamp_ticks", PropType::U64),
        ]
    }
}
