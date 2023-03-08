pub const CP437_WIDTH: usize = 16;
// Not needed because of u8 restrictions
// pub const CP437_HEIGHT: usize = 16;
pub const TILE_SIZE: usize = 20;
pub const SCREEN_WIDTH: usize = 20;
pub const SCREEN_HEIGHT: usize = 20;

pub type SdlResult = Result<(), Box<dyn std::error::Error>>;

pub mod entity;
pub use entity::*;

pub mod screen;
pub use screen::*;
