mod buffer;

use buffer::{Buffer, Color};

fn main() {
    let mut buffer = Buffer::new(1024, 1024);
    for i in 0..1024 {
        for j in 0..1024 {
            buffer.data[(i * 1024 + j) as usize] = Color::new(255, 0, 0, 255);
        }
    }

    buffer.save_to_file("output.png");
}
