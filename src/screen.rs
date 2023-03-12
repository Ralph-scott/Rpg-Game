use crate::*;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Glyph {
    x: usize,
    y: usize,
}

impl Glyph {
    pub const SIZE: usize = 20;
    pub const IMAGE_WIDTH: usize = 16;

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

    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        mut canvas: Canvas<Window>,
    ) -> SdlResult<Self> {
        canvas.set_blend_mode(BlendMode::None);
        let mut font = texture_creator.load_texture(Path::new("assets/sprites.png"))?;
        font.set_blend_mode(BlendMode::None);
        let screen = vec![Glyph::new(0); Self::WIDTH * Self::HEIGHT];

        Ok(Self {
            font,
            canvas,
            screen,
        })
    }

    pub fn set(&mut self, x: usize, y: usize, glyph: Glyph) {
        self.screen[y * Self::WIDTH + x] = glyph;
    }

    pub fn draw_text(&mut self, text: String, x: usize, y: usize) {
        for (i, ch) in text.bytes().enumerate() {
            if ch != b' ' {
                self.screen[y * Self::WIDTH + x + i] = Glyph::new(ch);
            }
        }
    }

    pub fn draw_dialogue(&mut self, text: String) {
        // TODO: Clean up this horrible mess
        for i in 1..5 {
            self.set(1, Screen::HEIGHT - 2 - i, Glyph::new(179));
            self.set(Screen::WIDTH - 2, Screen::HEIGHT - 2 - i, Glyph::new(179));
        }
        for i in 1..Screen::WIDTH - 3 {
            self.set(Screen::WIDTH - 2 - i, Screen::HEIGHT - 2, Glyph::new(196));
            self.set(Screen::WIDTH - 2 - i, Screen::HEIGHT - 7, Glyph::new(196));
        }
        self.set(Screen::WIDTH - 2, Screen::HEIGHT - 2, Glyph::new(217));
        self.set(1, Screen::HEIGHT - 2, Glyph::new(192));
        self.set(Screen::WIDTH - 2, Screen::HEIGHT - 7, Glyph::new(191));
        self.set(1, Screen::HEIGHT - 7, Glyph::new(218));
        for x in 2..Screen::WIDTH - 2 {
            for y in Screen::HEIGHT - 6..Screen::HEIGHT - 2 {
                self.set(x, y, Glyph::new(0));
            }
        }
        self.draw_text(text, 2, Screen::HEIGHT - 6);
    }

    pub fn draw(&mut self) -> SdlResult<()> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
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
