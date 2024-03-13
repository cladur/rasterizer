#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

pub struct Buffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            width,
            height,
            data: vec![Default::default(); (width * height) as usize],
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        let mut img = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self.data[(y * self.width + x) as usize];
            *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
        }
        img.save(filename).unwrap();
    }
}
