use crate::*;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Player {
    x: usize,
    y: usize,
}

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

pub struct World {
    player: Player,
    background: Vec<Sprite>,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player { x: 0, y: 0 },
            background: vec![
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
                Sprite::new(177),
            ],
        }
    }

    pub fn text(&mut self, text: String, x: usize, y: usize) {
        for (i, ch) in text.bytes().enumerate() {
            self.background[y * SCREEN_WIDTH + x + i] = Sprite::new(ch);
        }
    }

    pub fn update(&mut self, key: Keycode) {
        match key {
            Keycode::Up
            | Keycode::W
            | Keycode::K
            if self.player.y > 0 => {
                self.player.y -= 1;
            }
            Keycode::Down
            | Keycode::S
            | Keycode::J
            if self.player.y < SCREEN_HEIGHT - 1 => {
                self.player.y += 1;
            }
            Keycode::Left
            | Keycode::A
            | Keycode::H
            if self.player.x > 0 => {
                self.player.x -= 1;
            }
            Keycode::Right
            | Keycode::D
            | Keycode::L
            if self.player.x < SCREEN_WIDTH - 1 => {
                self.player.x += 1;
            }
            _ => {}
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, tilemap: &Texture<'_>) -> SdlResult {
        canvas.clear();
        for (i, sprite) in self.background.iter().enumerate() {
            let sprite = if i % SCREEN_WIDTH == self.player.x && i / SCREEN_WIDTH == self.player.y {
                Rect::new(10, 0, 10, 10)
            } else {
                Rect::new((sprite.x * 10) as i32, (sprite.y * 10) as i32, 10, 10)
            };
            canvas.copy(
                tilemap,
                Some(sprite),
                Some(Rect::new(
                    (i % SCREEN_WIDTH * TILE_SIZE) as i32,
                    (i / SCREEN_WIDTH * TILE_SIZE) as i32,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                )),
            )?;
        }
        Ok(())
    }
}
