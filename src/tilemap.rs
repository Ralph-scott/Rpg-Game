use crate::*;

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
            glyph: Glyph::new(178),
            kind: TileKind::Background,
        }
    }
}

pub struct Tilemap {
    pub tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn draw(&self, screen: &mut Screen<'_>) {
        for (i, tile) in self.tiles.iter().enumerate() {
            screen.screen[i] = tile.glyph;
        }
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
