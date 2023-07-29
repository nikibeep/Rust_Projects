extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: u64 = 3;

struct Stickman {
    x: i32,
    y: i32,
}

impl Stickman {
    fn new() -> Self {
        Stickman { x: 100, y: HEIGHT as i32 - 80 }
    }

    fn update(&mut self) {
        self.x = (self.x + 5) % WIDTH as i32;
    }

    fn draw(&self, buffer: &mut Vec<u32>) {
        let body_color = 0xFFFF0000; // Red
        let head_color = 0xFFFFFF00; // Yellow
        let limb_color = 0xFF0000FF; // Blue

        // Draw the head
        buffer[(self.y * WIDTH as i32 + self.x) as usize] = head_color;

        // Draw the body
        for y in self.y + 1..self.y + 40 {
            buffer[(y * WIDTH as i32 + self.x) as usize] = body_color;
        }

        // Draw the limbs
        buffer[((self.y + 5) * WIDTH as i32 + self.x + 5) as usize] = limb_color;
        buffer[((self.y + 5) * WIDTH as i32 + self.x - 5) as usize] = limb_color;
        buffer[((self.y + 35) * WIDTH as i32 + self.x + 5) as usize] = limb_color;
        buffer[((self.y + 35) * WIDTH as i32 + self.x - 5) as usize] = limb_color;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Running Stickman - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut stickman = Stickman::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        stickman.update();
        stickman.draw(&mut buffer);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        thread::sleep(Duration::from_millis(1000 / FPS));
    }
}
