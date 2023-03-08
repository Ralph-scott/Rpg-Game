use crate::*;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Player {
    x: usize,
    y: usize,
}

pub enum State {
    Menu,
    Overworld,
}

pub struct World {
    state: State,
    player: Player,
    background: Vec<Sprite>,
}

impl World {
    pub fn new() -> Self {
        Self {
            state: State::Menu,
            player: Player { x: 0, y: 0 },
            background: vec![Sprite::new(0); SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn update(&mut self, key: Keycode) {
        match self.state {
            State::Menu => match key {
                Keycode::Return => {
                    self.state = State::Overworld;
                }
                _ => {}
            },
            State::Overworld => match key {
                Keycode::Up | Keycode::W | Keycode::K if self.player.y > 0 => {
                    self.player.y -= 1;
                }
                Keycode::Down | Keycode::S | Keycode::J if self.player.y < SCREEN_HEIGHT - 1 => {
                    self.player.y += 1;
                }
                Keycode::Left | Keycode::A | Keycode::H if self.player.x > 0 => {
                    self.player.x -= 1;
                }
                Keycode::Right | Keycode::D | Keycode::L if self.player.x < SCREEN_WIDTH - 1 => {
                    self.player.x += 1;
                }
                _ => {}
            },
        }
    }

    pub fn draw_menu(&self, screen: &mut Screen) -> SdlResult {
        screen.draw_text(
            "\x01 Rpg game \x01".to_owned(),
            SCREEN_WIDTH / 2 - 6,
            SCREEN_HEIGHT / 2 - 3,
        );
        screen.draw_text(
            "Press enter to start".to_owned(),
            SCREEN_WIDTH / 2 - 10,
            SCREEN_HEIGHT / 2,
        );
        Ok(())
    }

    pub fn draw_playing(&self, screen: &mut Screen) -> SdlResult {
        screen.screen = self.background.clone();
        screen.set(self.player.x, self.player.y, 1);
        Ok(())
    }

    pub fn draw(&self, screen: &mut Screen, tilemap: &Texture<'_>) -> SdlResult {
        screen.clear();
        match self.state {
            State::Menu => self.draw_menu(screen)?,
            State::Overworld => self.draw_playing(screen)?,
        }
        screen.draw(tilemap)
    }
}
