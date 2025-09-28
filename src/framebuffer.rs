// framebuffer.rs
// Direct framebuffer output for TFT displays

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::io::{self, ErrorKind};

#[derive(Debug, Clone, Copy)]
pub enum FramebufferFormat {
    Rgb565Le,
    Rgb565Be,
    Rgb888,
    Bgr888,
    Rgba8888,
    Bgra8888,
}

impl FramebufferFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rgb565le" | "rgb565-little" | "rgb565" => Some(FramebufferFormat::Rgb565Le),
            "rgb565be" | "rgb565-big" => Some(FramebufferFormat::Rgb565Be),
            "rgb888" | "rgb24" => Some(FramebufferFormat::Rgb888),
            "bgr888" | "bgr24" => Some(FramebufferFormat::Bgr888),
            "rgba8888" | "rgba32" => Some(FramebufferFormat::Rgba8888),
            "bgra8888" | "bgra32" => Some(FramebufferFormat::Bgra8888),
            _ => None,
        }
    }

    pub fn bytes_per_pixel(self) -> usize {
        match self {
            FramebufferFormat::Rgb565Le | FramebufferFormat::Rgb565Be => 2,
            FramebufferFormat::Rgb888 | FramebufferFormat::Bgr888 => 3,
            FramebufferFormat::Rgba8888 | FramebufferFormat::Bgra8888 => 4,
        }
    }
}

pub struct FrameBuffer {
    file: std::fs::File,
    width: u32,
    height: u32,
    bytes_per_pixel: usize,
    format: FramebufferFormat,
}

impl FrameBuffer {
    // device_path can be configured by caller (main). format may be specified via PIXELSORT_FB_FMT.
    pub fn new(device_path: &str, width: u32, height: u32, format_override: Option<FramebufferFormat>) -> Result<Self, io::Error> {
        // Open device with read/write to validate permissions
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(device_path)
            .map_err(|e| io::Error::new(e.kind(), format!("Failed to open framebuffer device '{}': {}", device_path, e)))?;

        // Determine format: override or env var or default
        let format = if let Some(fmt) = format_override {
            fmt
        } else if let Ok(s) = std::env::var("PIXELSORT_FB_FMT") {
            FramebufferFormat::from_str(&s).unwrap_or(FramebufferFormat::Rgb565Le)
        } else {
            FramebufferFormat::Rgb565Le
        };

        let bytes_per_pixel = format.bytes_per_pixel();

        // Try to validate device size when possible. Note: many framebuffer devices report 0 length for special files.
        if let Ok(meta) = file.metadata() {
            let len = meta.len();
            if len > 0 {
                let expected = (width as u64) * (height as u64) * (bytes_per_pixel as u64);
                if len < expected {
                    return Err(io::Error::new(ErrorKind::Other, format!("Framebuffer device '{}' appears smaller ({}) than expected {} bytes for {}x{}@{}bpp. Check FB device or format (PIXELSORT_FB_FMT).", device_path, len, expected, width, height, bytes_per_pixel * 8)));
                }
            }
        }

        Ok(FrameBuffer { file, width, height, bytes_per_pixel, format })
    }

    pub fn write_frame(&mut self, pixels: &[u8]) -> Result<(), io::Error> {
        self.file.seek(SeekFrom::Start(0))?;

        let expected_src_len = (self.width as usize) * (self.height as usize) * 4; // source RGBA8
        if pixels.len() < expected_src_len {
            return Err(io::Error::new(ErrorKind::InvalidInput, format!("Source pixel buffer too small: {} < {}", pixels.len(), expected_src_len)));
        }

        // Convert source RGBA8 to requested framebuffer format
        match self.format {
            FramebufferFormat::Rgb565Le => {
                // Little-endian 16-bit
                let mut framebuffer_data = Vec::with_capacity((self.width as usize) * (self.height as usize) * 2);
                for chunk in pixels.chunks_exact(4) {
                    let r = chunk[0];
                    let g = chunk[1];
                    let b = chunk[2];
                    let rgb565: u16 = (((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | ((b as u16 & 0xF8) >> 3)) as u16;
                    // little endian
                    framebuffer_data.push((rgb565 & 0xFF) as u8);
                    framebuffer_data.push((rgb565 >> 8) as u8);
                }
                self.file.write_all(&framebuffer_data)?;
            }
            FramebufferFormat::Rgb565Be => {
                let mut framebuffer_data = Vec::with_capacity((self.width as usize) * (self.height as usize) * 2);
                for chunk in pixels.chunks_exact(4) {
                    let r = chunk[0];
                    let g = chunk[1];
                    let b = chunk[2];
                    let rgb565: u16 = (((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | ((b as u16 & 0xF8) >> 3)) as u16;
                    // big endian
                    framebuffer_data.push((rgb565 >> 8) as u8);
                    framebuffer_data.push((rgb565 & 0xFF) as u8);
                }
                self.file.write_all(&framebuffer_data)?;
            }
            FramebufferFormat::Rgb888 => {
                let mut framebuffer_data = Vec::with_capacity((self.width as usize) * (self.height as usize) * 3);
                for chunk in pixels.chunks_exact(4) {
                    framebuffer_data.push(chunk[0]); // r
                    framebuffer_data.push(chunk[1]); // g
                    framebuffer_data.push(chunk[2]); // b
                }
                self.file.write_all(&framebuffer_data)?;
            }
            FramebufferFormat::Bgr888 => {
                let mut framebuffer_data = Vec::with_capacity((self.width as usize) * (self.height as usize) * 3);
                for chunk in pixels.chunks_exact(4) {
                    framebuffer_data.push(chunk[2]); // b
                    framebuffer_data.push(chunk[1]); // g
                    framebuffer_data.push(chunk[0]); // r
                }
                self.file.write_all(&framebuffer_data)?;
            }
            FramebufferFormat::Rgba8888 => {
                // write RGBA directly
                self.file.write_all(pixels)?;
            }
            FramebufferFormat::Bgra8888 => {
                // swap R/B in-place into a buffer
                let mut framebuffer_data = Vec::with_capacity(pixels.len());
                for chunk in pixels.chunks_exact(4) {
                    framebuffer_data.push(chunk[2]); // b
                    framebuffer_data.push(chunk[1]); // g
                    framebuffer_data.push(chunk[0]); // r
                    framebuffer_data.push(chunk[3]); // a
                }
                self.file.write_all(&framebuffer_data)?;
            }
        }

        self.file.flush()?;
        
        // Force filesystem sync to ensure framebuffer is updated immediately
        self.file.sync_all().unwrap_or_default();
        
        Ok(())
    }
}
