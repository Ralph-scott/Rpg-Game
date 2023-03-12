use crate::*;
use std::fs;

pub fn load(file: &'static str) -> Tilemap {
    let tiles = fs::read_to_string(file).unwrap()
        .split_inclusive(",")
        .map(|x| {
            let mut chars = x.trim().trim_end_matches(",").chars();
            match chars.next().unwrap() {
                'B' => Tile {
                    glyph: Glyph::new(chars.as_str().parse().unwrap_or_else(|_| chars.next().unwrap() as u8)),
                    kind: TileKind::Background,
                },
                'S' => Tile {
                    glyph: Glyph::new(chars.as_str().parse().unwrap_or_else(|_| chars.next().unwrap() as u8)),
                    kind: TileKind::Solid,
                },
                'D' => todo!("Add doors"),
                _ => todo!()
            }
        })
        .collect();
    Tilemap { tiles }
}

#[derive(Debug, Copy, Clone)]
pub enum TileKind {
    Solid,
    Background,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub glyph: Glyph,
    pub kind: TileKind,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            glyph: Glyph::new(0),
            kind: TileKind::Background,
        }
    }
}

pub struct Tilemap {
    pub tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn draw(&self, screen: &mut Screen<'_>) {
        screen.screen = self.tiles.iter().map(|x| x.glyph).collect();
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * Screen::WIDTH + x]
    }

    pub fn wall_at(&self, x: usize, y: usize) -> bool {
        x as usize >= Screen::WIDTH
            || y as usize >= Screen::WIDTH
            || matches!(self.tile_at(x as usize, y as usize).kind, TileKind::Solid)
    }
}
