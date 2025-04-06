use minifb::{Key, Window, WindowOptions};
use tiny_skia::*;

mod fpga;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut window = Window::new("Rusty FPGA", width, height, WindowOptions::default())
        .expect("Unable to open window");

    let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        pixmap.fill(Color::from_rgba8(255, 255, 255, 255));

        let rect = Rect::from_xywh(50.0, 50.0, 100.0, 100.0).unwrap();
        let blue = {
            let mut paint = Paint::default();
            paint.set_color_rgba8(100, 100, 255, 255);
            paint
        };
        pixmap.fill_rect(rect, &blue, Transform::identity(), None);

        let buffer: Vec<u32> = pixmap
            .data()
            .chunks(4)
            .map(|px| {
                let r = px[0] as u32;
                let g = px[1] as u32;
                let b = px[2] as u32;
                (r << 16) | (g << 8) | b
            })
            .collect();

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}
