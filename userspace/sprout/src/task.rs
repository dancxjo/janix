use alloc::string::String;

#[derive(Debug, PartialEq)]
pub enum TaskKind {
    Driver(String), // Device Kind
    App,
}

pub struct ManagedTask {
    pub name: String,
    pub kind: TaskKind,
    #[allow(dead_code)]
    pub module_path: String,
    pub pid: Option<u64>,
    pub restarts: u32,
}
