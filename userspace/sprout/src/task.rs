use alloc::string::String;

#[derive(Debug, PartialEq)]
pub enum TaskKind {
    Driver(String), // Device Kind
    Service(String), // Service Kind
    App,
}

pub struct ManagedTask {
    pub name: String,
    pub kind: TaskKind,
    #[allow(dead_code)]
    pub module_path: String,
    pub pid: Option<u64>,
    pub restarts: u32,
    /// Original argument passed to spawn_process, preserved for restarts
    pub spawn_arg: usize,
}
