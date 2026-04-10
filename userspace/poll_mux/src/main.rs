#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall::vfs::*;
use abi::syscall::{PollFd, poll_flags};

#[no_main]
fn main() {
    stem::println!("--- poll_mux start ---");

    // 1. Create a pipe
    let mut pipefds = [0u32; 2];
    pipe(&mut pipefds).expect("pipe failed");
    let (pr, pw) = (pipefds[0], pipefds[1]);
    stem::println!("Pipe created: read={}, write={}", pr, pw);

    // 2. Create a channel and bridge it
    let (c1, c2) = stem::syscall::channel_create(1024).expect("channel create failed");
    let c1_fd = vfs_fd_from_handle(c1).expect("vfs_fd_from_handle failed");
    stem::println!("Channel created: h1={}, h2={}, bridged_fd={}", c1, c2, c1_fd);

    // 3. Open a device file
    let fb_fd = vfs_open("/dev/fb0", abi::syscall::vfs_flags::O_RDWR).expect("open fb0 failed");
    stem::println!("FB opened: fd={}", fb_fd);

    // 4. Test timeout (100ms)
    let mut fds = [
        PollFd { fd: pr as i32, events: poll_flags::POLLIN, revents: 0 },
        PollFd { fd: c1_fd as i32, events: poll_flags::POLLIN, revents: 0 },
    ];
    
    stem::println!("Polling for 100ms (should timeout)...");
    let start = stem::syscall::monotonic_ns();
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    let end = stem::syscall::monotonic_ns();
    stem::println!("Poll returned {} entries, took {} ms", n, (end - start) / 1_000_000);
    assert!(n == 0, "Expected timeout, got {}", n);

    // 5. Test pipe readiness (write before poll)
    vfs_write(pw, b"hello").expect("write to pipe failed");
    stem::println!("Wrote to pipe, polling now...");
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    stem::println!("Poll returned {} entries", n);
    assert!(n == 1, "Expected 1 ready entry, got {}", n);
    assert!(fds[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on pipe");

    // 6. Test channel readiness (write before poll)
    stem::syscall::channel_send(c2, b"world").expect("send to channel failed");
    stem::println!("Sent to channel, polling now...");
    fds[0].revents = 0;
    fds[1].revents = 0;
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    stem::println!("Poll returned {} entries", n);
    // Both should be ready now if order is preserved
    assert!(fds[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on pipe");
    assert!(fds[1].revents & poll_flags::POLLIN != 0, "Expected POLLIN on channel");

    stem::println!("--- poll_mux success ---");
    stem::syscall::exit(0);
}
