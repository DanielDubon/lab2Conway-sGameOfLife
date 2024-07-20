use crate::bmp::Bitmap;
use crate::color::Color;
use nalgebra_glm::Vec3;
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    const RED_COLOR: Color = Color { r: 255, g: 0, b: 0 };

    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0);
        let buffer = vec![background_color; width * height];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    pub fn get_point(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, hex_color: u32) {
        self.background_color = Color::from_hex(hex_color);
    }

    pub fn set_current_color(&mut self, hex_color: u32) {
        self.current_color = Color::from_hex(hex_color);
    }

   pub fn render_buffer(&self, filename: &str) -> std::io::Result<()> {
    let mut bitmap = Bitmap::new(self.width as u32, self.height as u32);

    for y in 0..self.height {
        let inverted_y = (self.height - y - 1) as u32;

        for x in 0..self.width {
            let color = self.buffer[y * self.width + x];
            bitmap.set_pixel(x as u32, inverted_y, (color.r, color.g, color.b));
        }
    }

    bitmap.save(filename)
}

    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }
        
        let index = (y as usize) * self.width + (x as usize);
        if index < self.buffer.len() {
            self.buffer[index] = color;
        }
    }

    pub fn fill_polygon(&mut self, vertices: &[Vec3], fill_color: Color) {
        
    }

    pub fn as_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| {
            (255 << 24) | // A
            ((color.r as u32) << 16) | // R
            ((color.g as u32) << 8) | // G
            (color.b as u32) // B
        }).collect()
    }
}
