pub type SdlResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod screen;
pub use screen::*;

pub mod tilemap;
pub use tilemap::*;

pub mod entity;
pub use entity::*;
