use crate::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sprite {
    x: usize,
    y: usize,
}

impl Sprite {
    pub fn new(ch: u8) -> Self {
        Self {
            x: ch as usize % CP437_WIDTH,
            y: ch as usize / CP437_WIDTH,
        }
    }
}

pub struct Screen {
    canvas: Canvas<Window>,
    pub screen: Vec<Sprite>,
}

impl Screen {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
            screen: vec![Sprite::new(0); SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.screen.fill(Sprite::new(0));
    }

    pub fn set(&mut self, x: usize, y: usize, ch: u8) {
        self.screen[y * SCREEN_WIDTH + x] = Sprite::new(ch);
    }

    pub fn draw_text(&mut self, text: String, x: usize, y: usize) {
        for (i, ch) in text.bytes().enumerate() {
            self.screen[y * SCREEN_WIDTH + x + i] = Sprite::new(ch);
        }
    }

    pub fn draw(&mut self, tilemap: &Texture<'_>) -> SdlResult {
        self.canvas.clear();
        for (i, sprite) in self.screen.iter().enumerate() {
            self.canvas.copy(
                tilemap,
                Some(Rect::new(
                    (sprite.x * 10) as i32,
                    (sprite.y * 10) as i32,
                    10,
                    10,
                )),
                Some(Rect::new(
                    (i % SCREEN_WIDTH * TILE_SIZE) as i32,
                    (i / SCREEN_WIDTH * TILE_SIZE) as i32,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                )),
            )?;
        }
        self.canvas.present();
        Ok(())
    }
}
