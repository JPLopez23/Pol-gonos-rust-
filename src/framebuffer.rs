use image::{Rgb, RgbImage};

pub struct Framebuffer {
    pub buffer: RgbImage,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = RgbImage::new(width, height);
        Self { buffer }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Rgb<u8>) {
        if x >= 0 && y >= 0 && (x as u32) < self.buffer.width() && (y as u32) < self.buffer.height() {
            self.buffer.put_pixel(x as u32, y as u32, color);
        }
    }

    pub fn save(&self, filename: &str) {
        self.buffer.save(filename).expect("Failed to save image");
    }
}