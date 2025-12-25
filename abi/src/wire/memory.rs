use crate::FrameId;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameInfo {
    pub id: FrameId,
    pub base: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MemorySummary {
    pub total_frames: u64,
    pub used_frames: u64,
    pub free_frames: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct SchedulerSummary {
    pub process_count: u64,
    pub thread_count: u64,
    pub runnable_threads: u64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MapFlags(pub u64);

impl MapFlags {
    pub const READ: MapFlags = MapFlags(1 << 0);
    pub const WRITE: MapFlags = MapFlags(1 << 1);
    pub const EXECUTE: MapFlags = MapFlags(1 << 2);
    pub const USER: MapFlags = MapFlags(1 << 3);
    pub const WRITE_COMBINE: MapFlags = MapFlags(1 << 4);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn contains(self, other: MapFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: MapFlags) -> MapFlags {
        MapFlags(self.0 | other.0)
    }
}

impl core::ops::BitOr for MapFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        MapFlags(self.0 | rhs.0)
    }
}
