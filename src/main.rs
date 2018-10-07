#[macro_use]
extern crate minifb;
extern crate nalgebra as na;

use minifb::{Key, WindowOptions, Window};

pub mod math;
pub mod controls;
pub mod scene;

use self::math::Ray;
use self::controls::Camera;
use self::scene::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];
    let mut window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e|{
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 1000;
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}