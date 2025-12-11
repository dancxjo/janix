use thing_models::{IoDirection, IoWidth};

#[derive(Debug)]
pub enum IoAccessError {
    Unsupported,
}

/// Perform an I/O port access in an architecture-specific way.
pub fn perform_io_operation(
    port_addr: u16,
    direction: IoDirection,
    width: IoWidth,
    value: u32,
) -> Result<Option<u32>, IoAccessError> {
    #[cfg(target_arch = "x86_64")]
    {
        use x86_64::instructions::port::Port;

        unsafe {
            match (direction, width) {
                (IoDirection::Write, IoWidth::U8) => {
                    Port::<u8>::new(port_addr).write(value as u8);
                    Ok(None)
                }
                (IoDirection::Write, IoWidth::U16) => {
                    Port::<u16>::new(port_addr).write(value as u16);
                    Ok(None)
                }
                (IoDirection::Write, IoWidth::U32) => {
                    Port::<u32>::new(port_addr).write(value);
                    Ok(None)
                }
                (IoDirection::Read, IoWidth::U8) => {
                    Ok(Some(Port::<u8>::new(port_addr).read() as u32))
                }
                (IoDirection::Read, IoWidth::U16) => {
                    Ok(Some(Port::<u16>::new(port_addr).read() as u32))
                }
                (IoDirection::Read, IoWidth::U32) => Ok(Some(Port::<u32>::new(port_addr).read())),
            }
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (port_addr, direction, width, value);
        Err(IoAccessError::Unsupported)
    }
}
