//! Draw command buffer for Wasm guests.
//!
//! This module provides a command buffer interface that maps to
//! embedded-graphics primitives. Wasm modules emit these commands,
//! and the host executes them against any DrawTarget.

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

/// Draw command opcodes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOpcode {
    /// No operation
    Nop = 0,
    /// Clear framebuffer: rgba (4 bytes)
    Clear = 1,
    /// Set pixel: x (i16), y (i16), rgba (4 bytes)
    Pixel = 2,
    /// Draw line: x0 (i16), y0 (i16), x1 (i16), y1 (i16), rgba (4 bytes)
    Line = 3,
    /// Draw circle outline: cx (i16), cy (i16), r (i16), rgba (4 bytes)
    Circle = 4,
    /// Draw filled circle: cx (i16), cy (i16), r (i16), rgba (4 bytes)
    FilledCircle = 5,
    /// Draw rectangle outline: x (i16), y (i16), w (i16), h (i16), rgba (4 bytes)
    Rect = 6,
    /// Draw filled rectangle: x (i16), y (i16), w (i16), h (i16), rgba (4 bytes)
    FilledRect = 7,
}

/// Maximum command buffer size (4 KiB)
pub const MAX_COMMAND_BUFFER_SIZE: usize = 4096;

/// Convert RGBA u32 to Rgb888 (ignore alpha for now)
fn rgba_to_rgb888(rgba: u32) -> Rgb888 {
    let r = ((rgba >> 24) & 0xFF) as u8;
    let g = ((rgba >> 16) & 0xFF) as u8;
    let b = ((rgba >> 8) & 0xFF) as u8;
    Rgb888::new(r, g, b)
}

/// Read a little-endian i16 from a byte slice
fn read_i16(bytes: &[u8]) -> i16 {
    if bytes.len() < 2 {
        return 0;
    }
    i16::from_le_bytes([bytes[0], bytes[1]])
}

/// Read a little-endian u32 from a byte slice
fn read_u32(bytes: &[u8]) -> u32 {
    if bytes.len() < 4 {
        return 0;
    }
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Execute a command buffer against any embedded-graphics DrawTarget.
///
/// Returns the number of commands successfully executed, or an error
/// if the buffer is malformed.
pub fn execute_commands<D>(target: &mut D, commands: &[u8]) -> Result<usize, CommandError>
where
    D: DrawTarget<Color = Rgb888>,
{
    let mut offset = 0;
    let mut count = 0;

    while offset < commands.len() {
        let opcode = commands[offset];
        offset += 1;

        match opcode {
            0 => {
                // Nop - skip
            }
            1 => {
                // Clear: 4 bytes (rgba)
                if offset + 4 > commands.len() {
                    return Err(CommandError::BufferUnderflow);
                }
                let _rgba = read_u32(&commands[offset..]);
                // Note: Clear would require DrawTarget extension or full-screen rect
                // For now, we skip actual clear and rely on host to clear
                offset += 4;
            }
            2 => {
                // Pixel: x (2), y (2), rgba (4) = 8 bytes
                if offset + 8 > commands.len() {
                    return Err(CommandError::BufferUnderflow);
                }
                let x = read_i16(&commands[offset..]) as i32;
                let y = read_i16(&commands[offset + 2..]) as i32;
                let rgba = read_u32(&commands[offset + 4..]);
                let color = rgba_to_rgb888(rgba);
                let _ = Pixel(Point::new(x, y), color).draw(target);
                offset += 8;
            }
            3 => {
                // Line: x0 (2), y0 (2), x1 (2), y1 (2), rgba (4) = 12 bytes
                if offset + 12 > commands.len() {
                    return Err(CommandError::BufferUnderflow);
                }
                let x0 = read_i16(&commands[offset..]) as i32;
                let y0 = read_i16(&commands[offset + 2..]) as i32;
                let x1 = read_i16(&commands[offset + 4..]) as i32;
                let y1 = read_i16(&commands[offset + 6..]) as i32;
                let rgba = read_u32(&commands[offset + 8..]);
                let color = rgba_to_rgb888(rgba);
                // Simple line via Bresenham would go here; for now draw endpoints
                let _ = Pixel(Point::new(x0, y0), color).draw(target);
                let _ = Pixel(Point::new(x1, y1), color).draw(target);
                offset += 12;
            }
            4 | 5 => {
                // Circle/FilledCircle: cx (2), cy (2), r (2), rgba (4) = 10 bytes
                if offset + 10 > commands.len() {
                    return Err(CommandError::BufferUnderflow);
                }
                let _cx = read_i16(&commands[offset..]) as i32;
                let _cy = read_i16(&commands[offset + 2..]) as i32;
                let _r = read_i16(&commands[offset + 4..]) as i32;
                let _rgba = read_u32(&commands[offset + 6..]);
                // Circle drawing would use embedded-graphics primitives
                // For now, placeholder
                offset += 10;
            }
            6 | 7 => {
                // Rect/FilledRect: x (2), y (2), w (2), h (2), rgba (4) = 12 bytes
                if offset + 12 > commands.len() {
                    return Err(CommandError::BufferUnderflow);
                }
                let _x = read_i16(&commands[offset..]) as i32;
                let _y = read_i16(&commands[offset + 2..]) as i32;
                let _w = read_i16(&commands[offset + 4..]) as i32;
                let _h = read_i16(&commands[offset + 6..]) as i32;
                let _rgba = read_u32(&commands[offset + 8..]);
                // Rectangle drawing via embedded-graphics
                // For now, placeholder
                offset += 12;
            }
            _ => {
                return Err(CommandError::UnknownOpcode(opcode));
            }
        }
        count += 1;
    }

    Ok(count)
}

/// Errors that can occur during command execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    /// Not enough bytes in buffer for command arguments
    BufferUnderflow,
    /// Unknown command opcode
    UnknownOpcode(u8),
}

/// Command buffer builder for Wasm guests
#[derive(Debug)]
pub struct CommandBuffer {
    buffer: [u8; MAX_COMMAND_BUFFER_SIZE],
    len: usize,
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBuffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; MAX_COMMAND_BUFFER_SIZE],
            len: 0,
        }
    }

    /// Reset the buffer
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Get the current buffer contents
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.len]
    }

    /// Append a pixel command
    pub fn pixel(&mut self, x: i16, y: i16, rgba: u32) -> bool {
        if self.len + 9 > MAX_COMMAND_BUFFER_SIZE {
            return false;
        }
        self.buffer[self.len] = CommandOpcode::Pixel as u8;
        self.len += 1;
        self.buffer[self.len..self.len + 2].copy_from_slice(&x.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 2].copy_from_slice(&y.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 4].copy_from_slice(&rgba.to_le_bytes());
        self.len += 4;
        true
    }

    /// Append a filled rectangle command
    pub fn filled_rect(&mut self, x: i16, y: i16, w: i16, h: i16, rgba: u32) -> bool {
        if self.len + 13 > MAX_COMMAND_BUFFER_SIZE {
            return false;
        }
        self.buffer[self.len] = CommandOpcode::FilledRect as u8;
        self.len += 1;
        self.buffer[self.len..self.len + 2].copy_from_slice(&x.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 2].copy_from_slice(&y.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 2].copy_from_slice(&w.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 2].copy_from_slice(&h.to_le_bytes());
        self.len += 2;
        self.buffer[self.len..self.len + 4].copy_from_slice(&rgba.to_le_bytes());
        self.len += 4;
        true
    }
}
