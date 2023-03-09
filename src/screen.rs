use crate::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use std::path::Path;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Glyph {
    x: usize,
    y: usize,
}

impl Glyph {
    pub const SIZE: usize = 20;
    pub const IMAGE_WIDTH: usize = 16;

    pub const SMILEY: Self = Self::new(1);

    pub const fn new(ch: u8) -> Self {
        Self {
            x: ch as usize % Self::IMAGE_WIDTH,
            y: ch as usize / Self::IMAGE_WIDTH,
        }
    }
}

pub struct Screen<'a> {
    font: Texture<'a>,
    canvas: Canvas<Window>,
    pub screen: Vec<Glyph>,
}

impl<'a> Screen<'a> {
    pub const WIDTH: usize = 20;
    pub const HEIGHT: usize = 20;

    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, canvas: Canvas<Window>) -> SdlResult<Self> {
        let font   = texture_creator.load_texture(Path::new("assets/sprites.png"))?;
        let screen = vec![Glyph::new(0); Self::WIDTH * Self::HEIGHT];

        Ok(Self { font, canvas, screen })
    }

    pub fn clear(&mut self) {
        self.screen.fill(Glyph::new(0));
    }

    pub fn set(&mut self, x: usize, y: usize, glyph: Glyph) {
        self.screen[y * Self::WIDTH + x] = glyph;
    }

    pub fn draw_text(&mut self, text: String, x: usize, y: usize) {
        for (i, ch) in text.bytes().enumerate() {
            self.screen[y * Self::WIDTH + x + i] = Glyph::new(ch);
        }
    }

    pub fn draw(&mut self) -> SdlResult<()> {
        self.canvas.clear();
        for (i, glyph) in self.screen.iter().enumerate() {
            self.canvas.copy(
                &self.font,
                Some(Rect::new(
                    (glyph.x * 10) as i32,
                    (glyph.y * 10) as i32,
                    10,
                    10,
                )),
                Some(Rect::new(
                    (i % Self::WIDTH * Glyph::SIZE) as i32,
                    (i / Self::WIDTH * Glyph::SIZE) as i32,
                    Glyph::SIZE as u32,
                    Glyph::SIZE as u32,
                )),
            )?;
        }
        self.canvas.present();
        Ok(())
    }
}
