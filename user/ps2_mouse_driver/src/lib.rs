#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use thing_models::{
    InterruptEvent, InterruptRequest, IoDirection, IoPortOp, IoPortRegion, IoStatus, IoWidth,
    MousePacketEvent,
};
use thing_os::prelude::*;

const POLL_INTERVAL_NS: u64 = 2_000_000;
const STATUS_OFFSET: u16 = 4;
const DATA_OFFSET: u16 = 0;
const MOUSE_IRQ_LINE: u8 = 12;
const MOUSE_RING_CAPACITY: usize = 128;

use abi::syscall_defs::{
    DevOpenArgs, DevOpenRet, DevReadArgs, DevReadRet, DeviceHandle, SysError, SysRet, UserPtr, UserSlice,
};
use abi::syscall_numbers::{SYS_DEV_OPEN, SYS_DEV_READ};
use thing_os::resident::{alloc_resident, ResidentObject};
use thing_os::resident::mouse::{MouseEntry, MouseStream};

unsafe fn syscall_dev_open(kind: u32, index: u32) -> Result<DeviceHandle, SysError> {
    let args = DevOpenArgs { kind, index };
    let mut ret = SysRet::<DevOpenRet> {
        ok: 0,
        val: DevOpenRet::default(),
        err: SysError { code: 0, detail: 0 },
    };
    
    core::arch::asm!(
        "int 0x80",
        in("rax") SYS_DEV_OPEN,
        in("rdi") &args,
        in("rsi") &ret,
        lateout("rcx") _,
        lateout("r11") _,
    );

    if ret.ok != 0 {
        Ok(ret.val.handle)
    } else {
        Err(ret.err)
    }
}

unsafe fn syscall_dev_read(handle: DeviceHandle, out: &mut [u8]) -> Result<usize, SysError> {
    let args = DevReadArgs {
        handle,
        out: UserSlice {
            ptr: UserPtr {
                addr: out.as_mut_ptr() as u64,
                _phantom: core::marker::PhantomData,
            },
            len: out.len() as u64,
        },
    };
    let mut ret = SysRet::<DevReadRet> {
        ok: 0,
        val: DevReadRet::default(),
        err: SysError { code: 0, detail: 0 },
    };

    core::arch::asm!(
        "int 0x80",
        in("rax") SYS_DEV_READ,
        in("rdi") &args,
        in("rsi") &ret,
        lateout("rcx") _,
        lateout("r11") _,
    );

    if ret.ok != 0 {
        Ok(ret.val.bytes_read as usize)
    } else {
        Err(ret.err)
    }
}

pub fn run_new_ABI<S: Sys>(sys: &mut S) -> ! {
    println(sys, "ps2_mouse_driver: starting (resident stream)");
    
    // 1. Allocate Resident Buffer
    let alloc = match alloc_resident(sys, "MouseStream", 65536) {
        Ok(r) => r,
        Err(e) => {
            let msg = alloc::format!("ps2_mouse_driver: resident alloc failed code={:?}", e.code);
            let leaked = alloc::boxed::Box::leak(msg.into_boxed_str());
            println(sys, leaked);
            loop { sys.sleep_for_ns(1_000_000_000); }
        }
    };
    
    let obj = unsafe { ResidentObject::<()>::new(alloc.thing_id, alloc.user_addr as *mut u8, alloc.byte_len as usize) };
    let mut stream = MouseStream::new(obj);
    stream.init(MOUSE_RING_CAPACITY as u32);
    
    // Advertise capabilities via props
    // We update "head" to 0 initially.
    let _ = update_props(sys, alloc.thing_id, &[
        ("head", PropValue::U64(0)),
        ("capacity", PropValue::U64(MOUSE_RING_CAPACITY as u64))
    ]);
    
    let region = wait_for_region(sys);
    println(sys, "ps2_mouse_driver: found i8042 IO region; initializing mouse port");

    let mut decoder = MouseDecoder::new(region.id, stream);
    
    let handle = match unsafe { syscall_dev_open(2, 0) } {
        Ok(h) => h,
        Err(e) => {
            let msg = alloc::format!("ps2_mouse_driver: failed to open device: code={}", e.code);
            let leaked = alloc::boxed::Box::leak(msg.into_boxed_str());
            println(sys, leaked);
            loop { sys.sleep_for_ns(1_000_000_000); }
        }
    };
    println(sys, "ps2_mouse_driver: device opened");

    let mut accessor = IoPortAccessor::new(region.id, Some(handle));
    if !init_mouse(sys, &mut accessor) {
        println(sys, "ps2_mouse_driver: mouse initialization failed");
    } else {
        println(sys, "ps2_mouse_driver: mouse initialization succeeded");
    }

    let mut buffer = [0u8; 16];

    loop {
        match unsafe { syscall_dev_read(handle, &mut buffer) } {
             Ok(count) => {
                if count > 0 {
                    for i in 0..count {
                         let byte = buffer[i];
                         decoder.process_byte(sys, byte);
                    }
                } else {
                    sys.sleep_for_ns(POLL_INTERVAL_NS);
                }
             }
             Err(_) => {
                sys.sleep_for_ns(POLL_INTERVAL_NS);
             }
        }
    }
}

fn wait_for_region<S: Sys>(sys: &mut S) -> IoPortRegion {
    loop {
        let regions: Vec<IoPortRegion> = list_things_by_kind(sys);
        if let Some(region) = regions.into_iter().find(|r| r.name == "i8042") {
            return region;
        }
        sys.sleep_for_ns(5_000_000);
    }
}

fn initial_interrupt_cursor<S: Sys>(sys: &mut S) -> u64 {
    list_things_by_kind::<S, InterruptEvent>(sys)
        .into_iter()
        .map(|event| event.id.0)
        .max()
        .unwrap_or(0)
}

fn init_mouse<S: Sys>(sys: &mut S, accessor: &mut IoPortAccessor) -> bool {
    if !accessor.command(sys, 0xA7) {
        return false;
    }
    accessor.flush_output(sys);

    // SKIP config byte setup (0x20 / 0x60) - assume keyboard driver did it.
    // If we try to read 0x20 response here, we might miss it if it goes to keyboard buffer.
    // And we blindly trust IRQ12 is enabled by keyboard driver.

    if !accessor.command(sys, 0xA8) {
        return false;
    }
    accessor.flush_output(sys);

    if !accessor.mouse_command(sys, 0xF6) {
        return false;
    }
    accessor.flush_output(sys);
    accessor.mouse_command(sys, 0xF4)
}

fn drain_mouse_bytes<S: Sys>(
    sys: &mut S,
    accessor: &mut IoPortAccessor,
    decoder: &mut MouseDecoder,
) {
    loop {
        match accessor.read_status(sys) {
            Some(status) if status & 0x01 != 0 => {
                if status & 0x20 == 0 {
                    break;
                }
                if let Some(byte) = accessor.read_data(sys) {
                    decoder.process_byte(sys, byte);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}

struct IoPortAccessor {
    region_id: ThingId,
    read_op: Option<ThingId>,
    write_op: Option<ThingId>,
    device_handle: Option<DeviceHandle>,
}

impl IoPortAccessor {
    fn new(region_id: ThingId, device_handle: Option<DeviceHandle>) -> Self {
        Self {
            region_id,
            read_op: None,
            write_op: None,
            device_handle,
        }
    }

    fn read_status<S: Sys>(&mut self, sys: &mut S) -> Option<u8> {
        self.read_u8(sys, STATUS_OFFSET)
    }

    fn read_data<S: Sys>(&mut self, sys: &mut S) -> Option<u8> {
        self.read_u8(sys, DATA_OFFSET)
    }

    fn write_data<S: Sys>(&mut self, sys: &mut S, value: u8) -> bool {
        self.write_u8(sys, DATA_OFFSET, value)
    }

    fn command<S: Sys>(&mut self, sys: &mut S, value: u8) -> bool {
        if !self.wait_input_clear(sys) {
            return false;
        }
        self.write_u8(sys, STATUS_OFFSET, value)
    }

    fn write_second_port<S: Sys>(&mut self, sys: &mut S, value: u8) -> bool {
        if !self.command(sys, 0xD4) {
            return false;
        }
        if !self.wait_input_clear(sys) {
            return false;
        }
        self.write_data(sys, value)
    }

    fn mouse_command<S: Sys>(&mut self, sys: &mut S, command: u8) -> bool {
        if !self.write_second_port(sys, command) {
            return false;
        }
        matches!(self.read_aux_data(sys), Some(0xFA))
    }

    fn read_aux_data<S: Sys>(&mut self, sys: &mut S) -> Option<u8> {
        let handle = self.device_handle?;
        let mut buffer = [0u8; 1];
        for _ in 0..200 {
            // Try to read one byte from kernel buffer using dev_read
            match unsafe { syscall_dev_read(handle, &mut buffer) } {
                Ok(1) => return Some(buffer[0]),
                Ok(_) => {
                    // Buffer empty, wait
                    sys.sleep_for_ns(100_000);
                }
                Err(_) => return None,
            }
        }
        None
    }

    fn flush_output<S: Sys>(&mut self, sys: &mut S) {
        // flush hardware buffer if any (via IoPortOp, just in case)
        while let Some(status) = self.read_status(sys) {
             if status & 0x01 == 0 { break; }
             let _ = self.read_data(sys);
        }
        // flush kernel buffer
        if let Some(handle) = self.device_handle {
             let mut buffer = [0u8; 16];
             loop {
                 match unsafe { syscall_dev_read(handle, &mut buffer) } {
                     Ok(0) | Err(_) => break,
                     Ok(_) => {}
                 }
             }
        }
    }

    fn wait_input_clear<S: Sys>(&mut self, sys: &mut S) -> bool {
        for _ in 0..100 {
            if let Some(status) = self.read_status(sys) {
                if status & 0x02 == 0 {
                    return true;
                }
            }
            sys.sleep_for_ns(100_000);
        }
        false
    }

    fn read_u8<S: Sys>(&mut self, sys: &mut S, offset: u16) -> Option<u8> {
        self.submit_op(sys, SlotKind::Read, offset, IoDirection::Read, 0)
            .map(|value| value as u8)
    }

    fn write_u8<S: Sys>(&mut self, sys: &mut S, offset: u16, value: u8) -> bool {
        self.submit_op(
            sys,
            SlotKind::Write,
            offset,
            IoDirection::Write,
            value as u32,
        )
        .is_some()
    }

    fn submit_op<S: Sys>(
        &mut self,
        sys: &mut S,
        slot_kind: SlotKind,
        offset: u16,
        direction: IoDirection,
        value: u32,
    ) -> Option<u32> {
        let region_id = self.region_id;
        let op_id = {
            let slot = self.slot(slot_kind);
            if let Some(id) = *slot {
                let props = [
                    ("offset", PropValue::U64(offset as u64)),
                    ("direction", PropValue::Str(direction.as_str().into())),
                    ("width", PropValue::Str(IoWidth::U8.as_str().into())),
                    ("value", PropValue::U64(value as u64)),
                    ("status", PropValue::Str(IoStatus::Pending.as_str().into())),
                ];
                if !update_props(sys, id, &props) {
                    return None;
                }
                id
            } else {
                let op =
                    IoPortOp::new(region_id, offset, direction, IoWidth::U8, value, ThingId(0));
                let Some(new_id) = create_thing(sys, &op) else {
                    return None;
                };
                *slot = Some(new_id);
                new_id
            }
        };

        self.wait_for_completion(sys, op_id)
    }

    fn slot(&mut self, kind: SlotKind) -> &mut Option<ThingId> {
        match kind {
            SlotKind::Read => &mut self.read_op,
            SlotKind::Write => &mut self.write_op,
        }
    }

    fn wait_for_completion<S: Sys>(&self, sys: &mut S, op_id: ThingId) -> Option<u32> {
        for _ in 0..200 {
            if let Some(op) = load_thing::<IoPortOp>(sys, op_id) {
                match op.status {
                    IoStatus::Completed => return Some(op.value),
                    IoStatus::Failed => return None,
                    _ => {}
                }
            }
            sys.sleep_for_ns(100_000);
        }
        None
    }
}

enum SlotKind {
    Read,
    Write,
}

struct MouseDecoder {
    controller_id: ThingId,
    sequence_index: u64,
    packet: [u8; 3],
    index: usize,
    stream: MouseStream<()>,
}

impl MouseDecoder {
    fn new(controller_id: ThingId, stream: MouseStream<()>) -> Self {
        Self {
            controller_id,
            sequence_index: 0,
            packet: [0; 3],
            index: 0,
            stream,
        }
    }

    fn process_byte<S: Sys>(&mut self, sys: &mut S, byte: u8) {
        if self.index == 0 && byte & 0x08 == 0 {
            return;
        }
        self.packet[self.index] = byte;
        self.index += 1;
        if self.index == 3 {
            self.index = 0;
            self.emit_event(sys);
        }
    }

    fn emit_event<S: Sys>(&mut self, sys: &mut S) {
        let status = self.packet[0];
        let dx = i16::from(self.packet[1] as i8);
        let dy = i16::from(self.packet[2] as i8);
        let buttons = status & 0x07;
        
        let timestamp = sys.time_monotonic_ns();
        
        // Append to ring
        let entry = MouseEntry {
            buttons,
            _pad0: 0,
            x: dx,
            y: dy,
            z: 0,
            timestamp,
        };
        self.stream.append(entry);
        
        // Signal update to wake compositor
        // We update 'head' property. We need 'head' value from stream?
        // stream.obj.header()...
        // Getting current head requires reading resident data.
        // But we just appended.
        // Actually, we can just bump a "dirty" counter or monotonic "head" in props.
        // Or just let update_props triggering the event be enough, value doesn't matter much if consumer checks ring.
        // But let's be accurate.
        // We know we incremented head. 
        // We can keep a local shadow head?
        
        // Simpler: Just update "head" with 0 or monotonic counter purely for WAKEUP.
        // The real head is in resident memory. The prop is just a signal.
        // But if someone reads prop, they might get confused.
        // Let's rely on the fact that Compositor looks at Resident memory.
        
        let _ = update_props(sys, self.stream.obj.id, &[("last_event_ns", PropValue::U64(timestamp))]);
    }
    
    fn next_sequence(&mut self) -> u64 {
        self.sequence_index += 1;
        self.sequence_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, Thing, ThingId};
    use alloc::vec::Vec;
    use thing_models::MousePacketEvent;
    use thing_os::doc_helpers::DocSys;

    fn find_prop(props: &'static [(PropKey, PropValue)], key: &str) -> Option<i64> {
        props.iter().find_map(|(k, v)| {
            if *k == key {
                match v {
                    PropValue::I64(v) => Some(*v),
                    PropValue::U64(v) => Some(*v as i64),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    #[test]
    fn mouse_decoder_emits_packet_event() {
        let mut sys =
            DocSys::with_responses(vec![KernelResponse::ThingCreated { id: ThingId(15) }]);
        let mut decoder = MouseDecoder::new(ThingId(2));
        decoder.process_byte(&mut sys, 0x08);
        decoder.process_byte(&mut sys, 5);
        decoder.process_byte(&mut sys, 0xFB);

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::ThingCreate { kind, props } if kind == &MousePacketEvent::KIND => {
                find_prop(props, "delta_x") == Some(5) && find_prop(props, "delta_y") == Some(-5)
            }
            _ => false,
        }));
    }

    #[test]
    fn mouse_decoder_ignores_packet_without_sync_bit() {
        let mut sys = DocSys::with_responses(Vec::new());
        let mut decoder = MouseDecoder::new(ThingId(2));
        decoder.process_byte(&mut sys, 0x00);
        assert!(sys.requests.borrow().is_empty());
    }
}
