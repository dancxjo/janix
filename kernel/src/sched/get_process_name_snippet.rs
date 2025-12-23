    pub fn get_process_name(&self, pid: ProcessId) -> Option<&'static str> {
        let index = process_index(pid);
        self.processes.get(index).and_then(|slot| slot.as_ref()).map(|p| p.name)
    }
