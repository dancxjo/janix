use std::sync::Mutex;

pub struct HostFrame {
    pub id: u64,
    #[allow(dead_code)]
    pub data: Box<[u8; 4096]>,
}

pub struct HostFramePool {
    frames: Vec<HostFrame>,
    used_count: usize,
}

impl HostFramePool {
    pub fn new(capacity: usize) -> Self {
        let mut frames = Vec::with_capacity(capacity);
        for i in 0..capacity {
            frames.push(HostFrame {
                id: i as u64 * 4096 + 0x100000, // Fake physical address
                data: Box::new([0; 4096]),
            });
        }
        Self {
            frames,
            used_count: 0,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<HostFrame> {
        if let Some(frame) = self.frames.pop() {
            self.used_count += 1;
            Some(frame)
        } else {
            None
        }
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        let free = self.frames.len() as u64;
        let used = self.used_count as u64;
        let total = free + used;
        (total, used, free)
    }
}

static HOST_FRAME_POOL: Mutex<Option<HostFramePool>> = Mutex::new(None);

pub fn init_host_frame_pool(capacity: usize) {
    *HOST_FRAME_POOL.lock().unwrap() = Some(HostFramePool::new(capacity));
}

pub fn allocate_frame() -> Option<HostFrame> {
    HOST_FRAME_POOL.lock().unwrap().as_mut()?.alloc_frame()
}

pub fn free_frame(_frame_id: u64) {
    let mut lock = HOST_FRAME_POOL.lock().unwrap();
    if let Some(pool) = lock.as_mut() {
        if pool.used_count > 0 {
            pool.used_count -= 1;
        }
    }
}

pub fn frame_stats() -> (u64, u64, u64) {
    HOST_FRAME_POOL
        .lock()
        .unwrap()
        .as_ref()
        .map(|p| p.stats())
        .unwrap_or((0, 0, 0))
}
