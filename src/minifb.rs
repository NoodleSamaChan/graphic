use minifb::{Window, WindowOptions, KeyRepeat};

use crate::Graphic;

pub struct Minifb {
    window: Window,
} 

impl Graphic for Minifb {
    fn new(name: &str, width: usize, height: usize) -> Self {

        let mut window = Window::new(
        name,
        width,
        height,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Minifb { window }

    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn is_key_down(&self, key: crate::Key) -> bool {
        self.window.is_key_down(key.to_minifb())
    }

    fn update_with_buffer(&mut self, windows: &window_rs::WindowBuffer) {
        self.window.update_with_buffer(&windows.buffer(), windows.width(), windows.height()).unwrap()
    }

    fn is_key_pressed(&self, key: crate::Key) -> bool {
        self.window.is_key_pressed(key.to_minifb(), KeyRepeat::Yes)
    }
}

impl crate::Key {
    fn to_minifb(&self) -> minifb::Key {
        match self {
            crate::Key::Up => minifb::Key::Up,
            crate::Key::Down => minifb::Key::Down,
            crate::Key::Left => minifb::Key::Left,
            crate::Key::Right => minifb::Key::Right,
            crate::Key::Escape => minifb::Key::Escape,
            crate::Key::Quit => minifb::Key::Q,
            crate::Key::Space => minifb::Key::Space,
            crate::Key::UpPlayer1 => minifb::Key::E,
            crate::Key::DownPlayer1 => minifb::Key::D,
            crate::Key::UpPlayer2 => minifb::Key::O,
            crate::Key::DownPlayer2 => minifb::Key::K,
            crate::Key::Launch => minifb::Key::W,
        }
    }
}