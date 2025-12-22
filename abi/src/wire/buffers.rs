use core::fmt;
use core::str::FromStr;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    Rgba8888 = 0,
    Bgra8888 = 1,
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelFormat::Rgba8888 => write!(f, "Rgba8888"),
            PixelFormat::Bgra8888 => write!(f, "Bgra8888"),
        }
    }
}

impl FromStr for PixelFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Rgba8888" => Ok(PixelFormat::Rgba8888),
            "Bgra8888" => Ok(PixelFormat::Bgra8888),
            _ => Err(()),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SharedBufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
}
