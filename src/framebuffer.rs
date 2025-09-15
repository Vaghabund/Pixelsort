// framebuffer.rs
// Direct framebuffer output for TFT displays

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

pub struct FrameBuffer {
    file: std::fs::File,
    width: u32,
    height: u32,
    bytes_per_pixel: usize,
}

impl FrameBuffer {
    pub fn new(device_path: &str, width: u32, height: u32) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(device_path)?;
            
        Ok(FrameBuffer {
            file,
            width,
            height,
            bytes_per_pixel: 4, // RGBA
        })
    }
    
    pub fn write_frame(&mut self, pixels: &[u8]) -> Result<(), std::io::Error> {
        self.file.seek(SeekFrom::Start(0))?;
        
        // Convert RGBA to RGB565 or RGB888 depending on display
        let mut framebuffer_data = Vec::new();
        
        for chunk in pixels.chunks_exact(4) {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            // Skip alpha for framebuffer
            
            // RGB565 format (2 bytes per pixel)
            let rgb565 = ((r as u16 & 0xF8) << 8) | 
                        ((g as u16 & 0xFC) << 3) | 
                        ((b as u16 & 0xF8) >> 3);
            
            framebuffer_data.push((rgb565 & 0xFF) as u8);
            framebuffer_data.push((rgb565 >> 8) as u8);
        }
        
        self.file.write_all(&framebuffer_data)?;
        self.file.flush()?;
        Ok(())
    }
}
