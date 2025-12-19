use crate::ThingId;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpawnProgramResult {
    pub process_id: ThingId,
    pub thread_id: ThingId,
}
