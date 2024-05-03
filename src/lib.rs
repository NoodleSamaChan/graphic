use window_rs::WindowBuffer;

pub trait Graphic {
    fn new(name: &str, width: usize, height: usize) -> Self;

    fn is_open(&self) -> bool;

    fn is_key_down(&self, key: Key) -> bool;

    fn update_with_buffer(&mut self, windows: &WindowBuffer);

    fn is_key_pressed(&self, key: Key) -> bool;

    fn get_keys_released(&self) -> Vec<Option<crate::Key>>;

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
    Launch,
}

#[cfg(feature = "minifb")]
pub mod minifb;
