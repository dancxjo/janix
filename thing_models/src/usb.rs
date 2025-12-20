use abi::{PropKey, PropValue, Thing, ThingId};
use crate::graph_kinds;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct UsbController {
    pub id: ThingId,
    pub name: String,
    pub pci_bus: u8,
    pub pci_slot: u8,
    pub pci_func: u8,
    pub mmio_base: u64,
}

impl Thing for UsbController {
    const KIND: &'static str = graph_kinds::KIND_USB_CONTROLLER;
    const DESCRIPTION: &'static str = "A USB Host Controller (e.g. XHCI)";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name".to_string(), PropValue::Str(self.name.clone())));
        out.push(("pci_bus".to_string(), PropValue::U64(self.pci_bus as u64)));
        out.push(("pci_slot".to_string(), PropValue::U64(self.pci_slot as u64)));
        out.push(("pci_func".to_string(), PropValue::U64(self.pci_func as u64)));
        out.push(("mmio_base".to_string(), PropValue::U64(self.mmio_base)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut pci_bus = 0;
        let mut pci_slot = 0;
        let mut pci_func = 0;
        let mut mmio_base = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "name" => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                "pci_bus" => {
                    if let PropValue::U64(v) = prop.1 {
                        pci_bus = v as u8;
                    }
                }
                "pci_slot" => {
                    if let PropValue::U64(v) = prop.1 {
                        pci_slot = v as u8;
                    }
                }
                "pci_func" => {
                    if let PropValue::U64(v) = prop.1 {
                        pci_func = v as u8;
                    }
                }
                "mmio_base" => {
                    if let PropValue::U64(v) = prop.1 {
                        mmio_base = v;
                    }
                }
                _ => {}
            }
        }
        Self {
            id,
            name,
            pci_bus,
            pci_slot,
            pci_func,
            mmio_base,
        }
    }

    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[]
    }
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub id: ThingId,
    pub controller_id: ThingId,
    pub slot: u8,
    pub address: u8,
    pub vid: u16,
    pub pid: u16,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
}

impl Thing for UsbDevice {
    const KIND: &'static str = graph_kinds::KIND_USB_DEVICE;
    const DESCRIPTION: &'static str = "A USB Device";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("controller_id".to_string(), PropValue::U64(self.controller_id.0)));
        out.push(("slot".to_string(), PropValue::U64(self.slot as u64)));
        out.push(("address".to_string(), PropValue::U64(self.address as u64)));
        out.push(("vid".to_string(), PropValue::U64(self.vid as u64)));
        out.push(("pid".to_string(), PropValue::U64(self.pid as u64)));
        out.push(("class".to_string(), PropValue::U64(self.class as u64)));
        out.push(("subclass".to_string(), PropValue::U64(self.subclass as u64)));
        out.push(("protocol".to_string(), PropValue::U64(self.protocol as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut controller_id = ThingId(0);
        let mut slot = 0;
        let mut address = 0;
        let mut vid = 0;
        let mut pid = 0;
        let mut class = 0;
        let mut subclass = 0;
        let mut protocol = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "controller_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        controller_id = ThingId(v);
                    }
                }
                "slot" => {
                    if let PropValue::U64(v) = prop.1 {
                        slot = v as u8;
                    }
                }
                "address" => {
                    if let PropValue::U64(v) = prop.1 {
                        address = v as u8;
                    }
                }
                "vid" => {
                    if let PropValue::U64(v) = prop.1 {
                        vid = v as u16;
                    }
                }
                "pid" => {
                    if let PropValue::U64(v) = prop.1 {
                        pid = v as u16;
                    }
                }
                "class" => {
                    if let PropValue::U64(v) = prop.1 {
                        class = v as u8;
                    }
                }
                "subclass" => {
                    if let PropValue::U64(v) = prop.1 {
                        subclass = v as u8;
                    }
                }
                "protocol" => {
                    if let PropValue::U64(v) = prop.1 {
                        protocol = v as u8;
                    }
                }
                _ => {}
            }
        }
        Self {
            id,
            controller_id,
            slot,
            address,
            vid,
            pid,
            class,
            subclass,
            protocol,
        }
    }

    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbTransferType {
    Control = 0,
    Interrupt = 1,
    Bulk = 2,
}

impl From<u64> for UsbTransferType {
    fn from(v: u64) -> Self {
        match v {
            0 => Self::Control,
            1 => Self::Interrupt,
            2 => Self::Bulk,
            _ => Self::Control, // Default
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    pub id: ThingId,
    pub device_id: ThingId,
    pub endpoint_number: u8,
    pub direction_in: bool,
    pub transfer_type: UsbTransferType,
    pub max_packet_size: u16,
    pub interval_ms: u8,
}

impl Thing for UsbEndpoint {
    const KIND: &'static str = graph_kinds::KIND_USB_ENDPOINT;
    const DESCRIPTION: &'static str = "A USB Endpoint";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("device_id".to_string(), PropValue::U64(self.device_id.0)));
        out.push(("endpoint_number".to_string(),
            PropValue::U64(self.endpoint_number as u64),
        ));
        out.push(("direction_in".to_string(), PropValue::Bool(self.direction_in)));
        out.push(("transfer_type".to_string(), PropValue::U64(self.transfer_type as u64)));
        out.push(("max_packet_size".to_string(),
            PropValue::U64(self.max_packet_size as u64),
        ));
        out.push(("interval_ms".to_string(), PropValue::U64(self.interval_ms as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut device_id = ThingId(0);
        let mut endpoint_number = 0;
        let mut direction_in = false;
        let mut transfer_type = UsbTransferType::Control;
        let mut max_packet_size = 0;
        let mut interval_ms = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "device_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        device_id = ThingId(v);
                    }
                }
                "endpoint_number" => {
                    if let PropValue::U64(v) = prop.1 {
                        endpoint_number = v as u8;
                    }
                }
                "direction_in" => {
                    if let PropValue::Bool(v) = prop.1 {
                        direction_in = v;
                    }
                }
                "transfer_type" => {
                    if let PropValue::U64(v) = prop.1 {
                        transfer_type = UsbTransferType::from(v);
                    }
                }
                "max_packet_size" => {
                    if let PropValue::U64(v) = prop.1 {
                        max_packet_size = v as u16;
                    }
                }
                "interval_ms" => {
                    if let PropValue::U64(v) = prop.1 {
                        interval_ms = v as u8;
                    }
                }
                _ => {}
            }
        }
        Self {
            id,
            device_id,
            endpoint_number,
            direction_in,
            transfer_type,
            max_packet_size,
            interval_ms,
        }
    }

    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbTransferKind {
    ControlSetup = 0,
    InterruptIn = 1,
    InterruptOut = 2,
}

impl From<u64> for UsbTransferKind {
    fn from(v: u64) -> Self {
        match v {
            0 => Self::ControlSetup,
            1 => Self::InterruptIn,
            2 => Self::InterruptOut,
            _ => Self::ControlSetup,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsbTransferRequest {
    pub id: ThingId,
    pub endpoint_id: ThingId,
    pub kind: UsbTransferKind,
    pub buffer: Vec<u8>,
    pub expected_len: u16,
    pub timeout_ms: u32,
}

impl Thing for UsbTransferRequest {
    const KIND: &'static str = graph_kinds::KIND_USB_TRANSFER_REQUEST;
    const DESCRIPTION: &'static str = "A request to perform a USB transfer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("endpoint_id".to_string(), PropValue::U64(self.endpoint_id.0)));
        out.push(("kind".to_string(), PropValue::U64(self.kind as u64)));
        // out.push(("buffer".to_string(), PropValue::Blob(self.buffer.clone())));
        out.push(("expected_len".to_string(), PropValue::U64(self.expected_len as u64)));
        out.push(("timeout_ms".to_string(), PropValue::U64(self.timeout_ms as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut endpoint_id = ThingId(0);
        let mut kind = UsbTransferKind::ControlSetup;
        let mut buffer = Vec::new();
        let mut expected_len = 0;
        let mut timeout_ms = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "endpoint_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        endpoint_id = ThingId(v);
                    }
                }
                "kind" => {
                    if let PropValue::U64(v) = prop.1 {
                        kind = UsbTransferKind::from(v);
                    }
                }
                // "buffer" => if let PropValue::Blob(v) = &prop.1 { buffer = v.clone(); },
                "expected_len" => {
                    if let PropValue::U64(v) = prop.1 {
                        expected_len = v as u16;
                    }
                }
                "timeout_ms" => {
                    if let PropValue::U64(v) = prop.1 {
                        timeout_ms = v as u32;
                    }
                }
                _ => {}
            }
        }
        Self {
            id,
            endpoint_id,
            kind,
            buffer,
            expected_len,
            timeout_ms,
        }
    }
    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbTransferStatus {
    Success = 0,
    Stall = 1,
    Timeout = 2,
    OtherError = 3,
}

impl From<u64> for UsbTransferStatus {
    fn from(v: u64) -> Self {
        match v {
            0 => Self::Success,
            1 => Self::Stall,
            2 => Self::Timeout,
            _ => Self::OtherError,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsbTransferResult {
    pub id: ThingId,
    pub request_id: ThingId,
    pub status: UsbTransferStatus,
    pub data: Vec<u8>,
}

impl Thing for UsbTransferResult {
    const KIND: &'static str = graph_kinds::KIND_USB_TRANSFER_RESULT;
    const DESCRIPTION: &'static str = "The result of a USB transfer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("request_id".to_string(), PropValue::U64(self.request_id.0)));
        out.push(("status".to_string(), PropValue::U64(self.status as u64)));
        // out.push(("data".to_string(), PropValue::Blob(self.data.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut request_id = ThingId(0);
        let mut status = UsbTransferStatus::OtherError;
        let mut data = Vec::new();

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "request_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        request_id = ThingId(v);
                    }
                }
                "status" => {
                    if let PropValue::U64(v) = prop.1 {
                        status = UsbTransferStatus::from(v);
                    }
                }
                // "data" => if let PropValue::Blob(v) = &prop.1 { data = v.clone(); },
                _ => {}
            }
        }
        Self {
            id,
            request_id,
            status,
            data,
        }
    }

    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[]
    }
}
