extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::string::String;
use core::fmt::Write;

use abi::display_protocol as dispproto;
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};
use stem::thing::{sys as thingsys, ThingId};
use stem::{thread, time};

use crate::asset::cursor::CursorTheme;
use crate::asset::wallpaper::decode_bmp;
use crate::asset::{apply_update, AssetHub, AssetJob, AssetJobQueue, AssetKind, AssetPack, AssetResultQueue, AssetUpdate};
use crate::compositor::Compositor;
use crate::present::{DisplaySurface, DriverPresenter, Presenter};
use crate::scene::SceneState;

const DISPLAY_ROLE_KEY: &str = "display_role";
const DISPLAY_ROLE_COMPOSITOR: &str = "display.compositor";

struct DisplayPorts {
    disp_req_read: PortHandle,
    disp_resp_write: PortHandle,
}

struct AssetRuntime {
    jobs: AssetJobQueue,
    results: AssetResultQueue,
}

static mut ASSET_RUNTIME: Option<&'static AssetRuntime> = None;

pub struct BlossomRuntime {
    display_ports: DisplayPorts,
    drv_req_write: PortHandle,
    drv_resp_read: PortHandle,
}

impl BlossomRuntime {
    pub fn new(arg: usize) -> Self {
        let disp_req_read = unpack_handle(arg, 0);
        let disp_resp_write = unpack_handle(arg, 1);
        let drv_req_write = unpack_handle(arg, 2);
        let drv_resp_read = unpack_handle(arg, 3);

        Self {
            display_ports: DisplayPorts {
                disp_req_read,
                disp_resp_write,
            },
            drv_req_write,
            drv_resp_read,
        }
    }

    pub fn run(self) -> ! {
        info!(
            "blossom: starting (disp_req_r={}, disp_resp_w={}, drv_req_w={}, drv_resp_r={})",
            self.display_ports.disp_req_read,
            self.display_ports.disp_resp_write,
            self.drv_req_write,
            self.drv_resp_read
        );

        let surface = match find_compositor_surface() {
            Some(surface) => surface,
            None => {
                info!("blossom: compositor bytespace not found");
                loop {
                    stem::yield_now();
                }
            }
        };

        let bytespace_ptr = match thingsys::bytespace_map(ThingId(surface.bytespace_id)) {
            Ok(ptr) => ptr as *mut u8,
            Err(e) => {
                info!("blossom: bytespace_map failed: {:?}", e);
                loop {
                    stem::yield_now();
                }
            }
        };

        let display_surface = DisplaySurface {
            bytespace_id: surface.bytespace_id,
            width: surface.width,
            height: surface.height,
            stride: surface.stride,
            format: surface.format,
        };

        let mut presenter = DriverPresenter::new(
            display_surface,
            bytespace_ptr,
            self.drv_req_write,
            self.drv_resp_read,
        );

        let compositor = Compositor;
        let hub = AssetHub::new(AssetPack::placeholder());

        let asset_runtime = Box::leak(Box::new(AssetRuntime {
            jobs: AssetJobQueue::new(),
            results: AssetResultQueue::new(),
        }));
        unsafe {
            ASSET_RUNTIME = Some(asset_runtime);
        }

        spawn_asset_workers();
        queue_default_assets(asset_runtime);
        info!("blossom: render loop started");

        let mut cursor_generation = 0u64;
        let mut cursor_start_ns = time::monotonic_ns();

        let mut disp_buf = [0u8; 512];
        let mut disp_rx_buf = [0u8; 1024];
        let mut disp_rx_len = 0usize;

        loop {
            presenter.poll_driver();
            poll_display_protocol(&self.display_ports, &surface, &mut disp_buf, &mut disp_rx_buf, &mut disp_rx_len);

            while let Some(update) = asset_runtime.results.pop() {
                let label = match &update {
                    AssetUpdate::WallpaperReady(_) => "wallpaper",
                    AssetUpdate::CursorReady(_) => "cursor",
                };
                let current = hub.current_pack();
                let next_gen = hub.next_generation(current.generation);
                let next = apply_update(&current, update, next_gen);
                if next.generation != current.generation {
                    if next.generation != cursor_generation {
                        cursor_start_ns = time::monotonic_ns();
                        cursor_generation = next.generation;
                    }
                    info!("blossom: {} ready gen={}", label, next.generation);
                    hub.submit_ready_pack(next);
                }
            }

            let now_ns = time::monotonic_ns();
            let assets = hub.current_pack();
            if assets.generation != cursor_generation {
                cursor_generation = assets.generation;
                cursor_start_ns = now_ns;
            }

            let cursor_pos = (
                (surface.width as i32) / 2,
                (surface.height as i32) / 2,
            );

            let scene = SceneState {
                screen_width: surface.width,
                screen_height: surface.height,
                cursor_pos,
                assets: Arc::clone(&assets),
                now_ns,
            };

            let elapsed_ms = (now_ns.saturating_sub(cursor_start_ns)) / 1_000_000;
            let cursor_frame = scene.assets.cursor.frame_at_ms(elapsed_ms);
            let drawlist = Some(bloom::demo_drawlist(scene.screen_width as i32, scene.screen_height as i32));

            {
                let mut surface = presenter.surface_mut();
                compositor.compose(&scene, &mut surface, cursor_frame, drawlist.as_ref());
            }
            presenter.present();

            let frame_ms = presenter.vsync_hint().unwrap_or(16) as u64;
            time::sleep_ms(frame_ms);
        }
    }
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn send_display_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = dispproto::encode_message(&mut buf, msg_type, payload) {
        let _ = port_send(handle, &buf[..len]);
    }
}

fn parse_disp_header(buf: &[u8]) -> Option<dispproto::DisplayHeader> {
    if buf.len() < dispproto::HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let msg_type = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    Some(dispproto::DisplayHeader {
        magic,
        version,
        msg_type,
        payload_len,
    })
}

fn poll_display_protocol(
    ports: &DisplayPorts,
    surface: &DisplaySurface,
    disp_buf: &mut [u8],
    disp_rx_buf: &mut [u8],
    disp_rx_len: &mut usize,
) {
    if let Ok(n) = port_recv(ports.disp_req_read, disp_buf) {
        if n > 0 {
            if *disp_rx_len + n > disp_rx_buf.len() {
                *disp_rx_len = 0;
            }
            disp_rx_buf[*disp_rx_len..*disp_rx_len + n].copy_from_slice(&disp_buf[..n]);
            *disp_rx_len += n;
        }
    }

    while *disp_rx_len >= dispproto::HEADER_SIZE {
        let header = match parse_disp_header(&disp_rx_buf[..*disp_rx_len]) {
            Some(header) => header,
            None => {
                *disp_rx_len = 0;
                break;
            }
        };

        if header.magic != dispproto::DISPLAY_MAGIC || header.version != dispproto::DISPLAY_VERSION {
            disp_rx_buf.copy_within(1..*disp_rx_len, 0);
            *disp_rx_len -= 1;
            continue;
        }

        let total = dispproto::HEADER_SIZE + (header.payload_len as usize);
        if *disp_rx_len < total {
            break;
        }

        let payload = &disp_rx_buf[dispproto::HEADER_SIZE..total];
        match header.msg_type {
            dispproto::MSG_HELLO => {
                send_display_msg(ports.disp_resp_write, dispproto::MSG_ACK, &[]);
            }
            dispproto::MSG_INFO_REQ => {
                let info = dispproto::InfoResp {
                    width: surface.width,
                    height: surface.height,
                    stride: surface.stride,
                    format: surface.format,
                };
                let info_bytes = unsafe {
                    core::slice::from_raw_parts(
                        &info as *const _ as *const u8,
                        core::mem::size_of::<dispproto::InfoResp>(),
                    )
                };
                send_display_msg(ports.disp_resp_write, dispproto::MSG_INFO_RESP, info_bytes);
            }
            dispproto::MSG_BUFFER_REQ => {
                let size = (surface.height as u64) * (surface.stride as u64);
                let resp = dispproto::BufferResp {
                    bytespace_id: surface.bytespace_id,
                    size,
                    stride: surface.stride,
                    format: surface.format,
                };
                let resp_bytes = unsafe {
                    core::slice::from_raw_parts(
                        &resp as *const _ as *const u8,
                        core::mem::size_of::<dispproto::BufferResp>(),
                    )
                };
                send_display_msg(ports.disp_resp_write, dispproto::MSG_BUFFER_RESP, resp_bytes);
            }
            dispproto::MSG_PRESENT => {
                if payload.len() >= core::mem::size_of::<dispproto::PresentHeader>() {
                    send_display_msg(ports.disp_resp_write, dispproto::MSG_ACK, &[]);
                } else {
                    send_display_msg(ports.disp_resp_write, dispproto::MSG_ACK, &[]);
                }
            }
            _ => {}
        }

        if total < *disp_rx_len {
            disp_rx_buf.copy_within(total..*disp_rx_len, 0);
        }
        *disp_rx_len -= total;
    }
}

fn find_compositor_surface() -> Option<DisplaySurface> {
    let role_sym = thingsys::intern(DISPLAY_ROLE_COMPOSITOR).ok()?;

    let mut buf = [ThingId(0); 64];
    let count = thingsys::find(abi::schema::kinds::BYTESPACE, &mut buf).ok()?;
    for i in 0..core::cmp::min(count, buf.len()) {
        let bs = buf[i];
        if bs.0 == 0 {
            continue;
        }
        let role = match thingsys::prop_get(bs, DISPLAY_ROLE_KEY) {
            Ok(val) => val,
            Err(_) => continue,
        };
        if role != role_sym as u64 {
            continue;
        }

        let width = thingsys::prop_get(bs, abi::schema::keys::WIDTH).unwrap_or(0) as u32;
        let height = thingsys::prop_get(bs, abi::schema::keys::HEIGHT).unwrap_or(0) as u32;
        let stride = thingsys::prop_get(bs, abi::schema::keys::STRIDE).unwrap_or(0) as u32;
        let format = thingsys::prop_get(bs, abi::schema::keys::FORMAT).unwrap_or(0) as u32;
        return Some(DisplaySurface {
            bytespace_id: bs.0,
            width,
            height,
            stride,
            format,
        });
    }

    None
}

fn spawn_asset_workers() {
    if let Ok(tid) = thread::spawn(asset_worker_main) {
        info!("blossom: asset worker started tid={}", tid);
    }
    if let Ok(tid) = thread::spawn(asset_worker_main) {
        info!("blossom: asset worker started tid={}", tid);
    }
}

fn queue_default_assets(runtime: &AssetRuntime) {
    let wallpaper_bytes = include_bytes!("../../../assets/wallpapers/clouds.bmp");
    let cursor_bytes = include_bytes!("../../../assets/cursors/plain/Normal.cur");

    runtime.jobs.push(AssetJob {
        kind: AssetKind::Wallpaper,
        bytes: wallpaper_bytes,
        name: "clouds.bmp",
    });
    runtime.jobs.push(AssetJob {
        kind: AssetKind::Cursor,
        bytes: cursor_bytes,
        name: "Normal.cur",
    });
    info!("blossom: wallpaper job queued");
    info!("blossom: cursor job queued");
}

extern "C" fn asset_worker_main() -> ! {
    loop {
        let job = unsafe { ASSET_RUNTIME.and_then(|rt| rt.jobs.pop()) };
        if let Some(job) = job {
            log_asset_header(job.name, job.bytes);
            match job.kind {
                AssetKind::Wallpaper => {
                    match decode_bmp(job.bytes) {
                        Ok(wallpaper) => {
                            info!(
                                "blossom: wallpaper decoded ok ({}x{})",
                                wallpaper.width, wallpaper.height
                            );
                            let update = AssetUpdate::WallpaperReady(Arc::new(wallpaper));
                            unsafe {
                                if let Some(rt) = ASSET_RUNTIME {
                                    rt.results.push(update);
                                }
                            }
                        }
                        Err(err) => {
                            info!("blossom: wallpaper decode failed: {:?}; using fallback", err);
                            let fallback = crate::asset::wallpaper::WallpaperSurface::error_fallback();
                            let update = AssetUpdate::WallpaperReady(Arc::new(fallback));
                            unsafe {
                                if let Some(rt) = ASSET_RUNTIME {
                                    rt.results.push(update);
                                }
                            }
                        }
                    }
                }
                AssetKind::Cursor => {
                    if let Some(theme) = CursorTheme::from_bytes(job.bytes) {
                        let update = AssetUpdate::CursorReady(Arc::new(theme));
                        unsafe {
                            if let Some(rt) = ASSET_RUNTIME {
                                rt.results.push(update);
                            }
                        }
                    } else {
                        info!("blossom: cursor decode failed");
                    }
                }
            }
        } else {
            thread::yield_now();
        }
    }
}

fn log_asset_header(name: &str, bytes: &[u8]) {
    let take = bytes.len().min(16);
    let mut header = [0u8; 16];
    if take > 0 {
        header[..take].copy_from_slice(&bytes[..take]);
    }
    let mut hex = String::new();
    if hex.try_reserve(take.saturating_mul(3)).is_ok() {
        for i in 0..take {
            let _ = write!(
                &mut hex,
                "{:02X}{}",
                header[i],
                if i + 1 == take { "" } else { " " }
            );
        }
    }
    info!(
        "blossom: asset='{}' len={} header=[{}]",
        name,
        bytes.len(),
        hex
    );
}
