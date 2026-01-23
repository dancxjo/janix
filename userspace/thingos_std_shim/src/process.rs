pub fn exit(code: i32) -> ! {
    stem::syscall::exit(code)
}

pub fn abort() -> ! {
    stem::syscall::exit(134)
}
