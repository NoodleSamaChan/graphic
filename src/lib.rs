use window_rs::WindowBuffer;

pub trait Graphic {
    fn new(name: &str, width: usize, height: usize) -> Self;

    fn is_open(&self) -> bool;

    fn is_key_down(&self, key: Key) -> bool;

    fn update_with_buffer(&mut self, windows: &WindowBuffer);

    fn is_key_pressed(&self, key: Key) -> bool;

    fn get_keys_released(&self) -> Vec<crate::Key>;

    fn get_mouse_pos(&self, mouse: Mouse) -> Option<(f32, f32)>;

    fn get_mouse_down(&self, mouse: Mouse) -> bool;

}

pub enum Mouse {
    Left,
    Right,
    Discard,
}

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Quit,
    Space,
    UpPlayer1,
    DownPlayer1,
    UpPlayer2,
    DownPlayer2,
    LeftPlayer2,
    RightPlayer2,
    Launch,
    Forward,
    Backward,
    Save,
}

#[cfg(feature = "minifb")]
pub mod minifb;
