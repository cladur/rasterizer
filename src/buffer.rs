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

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
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

    fn barycentric(&self, points: &[Vec2; 3], p: Vec2) -> Vec3 {
        let u = Vec3::new(
            points[2].x - points[0].x,
            points[1].x - points[0].x,
            points[0].x - p.x,
        );
        let v = Vec3::new(
            points[2].y - points[0].y,
            points[1].y - points[0].y,
            points[0].y - p.y,
        );
        let u_cross_v = Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );
        if u_cross_v.z.abs() < 1.0 {
            return Vec3::new(-1.0, 1.0, 1.0);
        }
        Vec3::new(
            1.0 - (u_cross_v.x + u_cross_v.y) / u_cross_v.z,
            u_cross_v.x / u_cross_v.z,
            u_cross_v.y / u_cross_v.z,
        )
    }

    pub fn draw_triangle(&mut self, points: &[Vec2; 3], color: Color) {
        // Points are given in NDC space
        // Convert to screen space
        let mut points_screen = [Vec2::new(0.0, 0.0); 3];
        for i in 0..3 {
            points_screen[i].x = (points[i].x + 1.0) * (self.width as f32) / 2.0;
            points_screen[i].y = (points[i].y + 1.0) * (self.height as f32) / 2.0;
        }

        let min_x = points_screen[0]
            .x
            .min(points_screen[1].x.min(points_screen[2].x)) as u32;
        let max_x = points_screen[0]
            .x
            .max(points_screen[1].x.max(points_screen[2].x)) as u32;
        let min_y = points_screen[0]
            .y
            .min(points_screen[1].y.min(points_screen[2].y)) as u32;
        let max_y = points_screen[0]
            .y
            .max(points_screen[1].y.max(points_screen[2].y)) as u32;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let p = Vec2::new(x as f32, y as f32);
                let bary = self.barycentric(&points_screen, p);
                if bary.x < 0.0 || bary.y < 0.0 || bary.z < 0.0 {
                    continue;
                }
                self.data[(y * self.width + x) as usize] = color;
            }
        }
    }
}
