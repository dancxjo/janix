use abi::ThingId;
use kernel::sched::SCHEDULER;
use kernel::sched_types::CpuId;
use spin::Mutex;

const MAX_CPUS: usize = 4;
static CURRENT_THREADS: Mutex<[Option<ThingId>; MAX_CPUS]> = Mutex::new([None; MAX_CPUS]);

pub fn arch_current_thread(cpu: CpuId) -> Option<ThingId> {
    let idx = cpu as usize;
    if idx >= MAX_CPUS {
        return None;
    }
    let slots = CURRENT_THREADS.lock();
    slots[idx]
}

pub fn arch_switch_to_thread(cpu: CpuId, next: ThingId) {
    let idx = cpu as usize;
    if idx >= MAX_CPUS {
        kernel::println!(
            "arch_switch_to_thread: attributionem Thread({}) ad CpuCore({}) neglego",
            next.0,
            cpu
        );
        return;
    }

    let previous = {
        let mut slots = CURRENT_THREADS.lock();
        let prev = slots[idx];
        if prev == Some(next) {
            return;
        }
        slots[idx] = Some(next);
        prev
    };

    match previous {
        Some(prev) => {
            kernel::println!("CPU {}: Fila({}) -> Fila({})", cpu, prev.0, next.0);
        }
        None => {
            kernel::println!("CPU {}: incipit Fila({})", cpu, next.0);
        }
    }

    let sched = SCHEDULER.lock();
    let tid = sched.thread_id_for_thing(next).map(|id| id.0);
    if let Some(thread) = sched.thread_by_thing(next) {
        let tid_num = tid.unwrap_or(thread.id.0);
        kernel::println!("CPU {} nunc currit {} (tid {})", cpu, thread.name, tid_num);
    } else {
        kernel::println!(
            "CPU {}: Fila({}) in repositorio schedulatoris non inventa",
            cpu,
            next.0
        );
    }
    drop(sched);

    // TODO: Once arch contexts are wired up, save/restore registers here.
}
