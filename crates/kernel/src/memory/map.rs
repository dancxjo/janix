use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MapPerms: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXEC = 1 << 2;
        const USER = 1 << 3;
        const DEVICE = 1 << 4;
    }
}

#[derive(Debug)]
pub enum MapError {
    InvalidAddress,
    Overlap,
    Oom,
    GraphError,
}

pub type MapResult<T> = Result<T, MapError>;
