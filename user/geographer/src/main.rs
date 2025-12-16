#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;

    use examples_support::{log, sleep_ms, draw_text_simple, SimpleWindow, UserlandSys};
    use alloc::format;
    use alloc::vec::Vec;
    use abi::{PropType, PropKey, PropValue, Predicate, Thing, ThingId};
    use thing_os::list_things_by_kind;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(_) = run(&mut sys) {
            log(&mut sys, "geographer", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "geographer", "starting");

        // 1. Create a Pixel-Based Window
        let mut win = SimpleWindow::new(sys, "Geographer").ok_or(())?;
        log(sys, "geographer", "window created");

        let mut nodes: Vec<ThingId> = Vec::new();
        let mut links: Vec<LinkThing> = Vec::new();

        loop {
            // --- 2. Update Graph State ---
            nodes.clear();
            links.clear();

            // Simple Discovery
            let processes: Vec<ProcessThing> = list_things_by_kind(sys);
            for p in processes {
                nodes.push(p.id);
            }
            
            // Just counting links for now
            let all_links: Vec<LinkThing> = list_things_by_kind(sys);
            for l in all_links {
                links.push(l);
            }

            // --- 3. Render ---
            // Clear background
            fill_rect(win.buffer, win.stride, win.width, win.height, 0, 0, win.width as i32, win.height as i32, 0xFF202020);

            // Header
            let stats = format!("System Graph | Nodes: {}  Links: {}", nodes.len(), links.len());
            draw_text_simple(win.buffer, win.stride, win.width, win.height, 10, 10, &stats, 0xFFFFFFFF);

            // List Nodes
            let mut y = 40;
            for (i, node) in nodes.iter().enumerate() {
                if y > win.height as i32 - 20 { break; }
                let s = format!("Process Node #{}: ID({})", i, node.0);
                draw_text_simple(win.buffer, win.stride, win.width, win.height, 20, y, &s, 0xFF00FF00);
                y += 20;
            }

            sleep_ms(sys, 1000);
        }
    }

    // --- Minimal Graph Queries ---

    pub struct LinkThing {
        pub id: ThingId,
    }

    impl Thing for LinkThing {
        const KIND: &'static str = "Link";
        const DESCRIPTION: &'static str = "";
        fn schema() -> &'static [(&'static str, PropType)] { &[] }
        fn to_props(&self, _: &mut Vec<(PropKey, PropValue)>) {}
        fn from_props(id: ThingId, _: &[Option<(PropKey, PropValue)>]) -> Self {
            LinkThing { id }
        }
    }

    pub struct ProcessThing {
        pub id: ThingId,
    }
    impl Thing for ProcessThing {
        const KIND: &'static str = "Process";
        const DESCRIPTION: &'static str = "";
        fn schema() -> &'static [(&'static str, PropType)] { &[] }
        fn to_props(&self, _: &mut Vec<(PropKey, PropValue)>) {}
        fn from_props(id: ThingId, _: &[Option<(PropKey, PropValue)>]) -> Self { ProcessThing { id } }
    }

    // --- Drawing Helper ---

    fn fill_rect(buffer: &mut [u8], stride: u32, _width: u32, height: u32, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let b_r = (color & 0xFF) as u8;
        let b_g = ((color >> 8) & 0xFF) as u8;
        let b_b = ((color >> 16) & 0xFF) as u8;
        let b_a = ((color >> 24) & 0xFF) as u8;

        for row in 0..h {
            let py = y + row;
            if py < 0 || py as u32 >= height { continue; }
            for col in 0..w {
                let px = x + col;
                if px < 0 { continue; }
                let offset = (py as u32 * stride + px as u32 * 4) as usize;
                if offset + 4 <= buffer.len() {
                    buffer[offset] = b_r;
                    buffer[offset + 1] = b_g;
                    buffer[offset + 2] = b_b;
                    buffer[offset + 3] = b_a;
                }
            }
        }
    }
}

// Host shim
#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
