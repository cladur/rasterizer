mod buffer;

use buffer::{Buffer, Color, Vec2};

fn main() {
    let mut buffer = Buffer::new(1024, 1024);
    for i in 0..1024 {
        for j in 0..1024 {
            buffer.data[(i * 1024 + j) as usize] = Color::new(0, 0, 0, 255);
        }
    }

    buffer.draw_triangle(
        &[
            Vec2::new(-0.6, -0.3),
            Vec2::new(0.4, -0.1),
            Vec2::new(-0.2, 0.6),
        ],
        Color::new(255, 255, 255, 255),
    );

    // buffer.draw_triangle(
    //     &[
    //         Vec2::new(0.1, 0.1),
    //         Vec2::new(0.6, 0.2),
    //         Vec2::new(0.2, 0.7),
    //     ],
    //     Color::new(255, 0, 0, 255),
    // );

    buffer.save_to_file("output.png");
}
