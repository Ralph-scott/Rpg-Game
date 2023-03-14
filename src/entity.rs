use crate::*;
use sdl2::keyboard::Keycode;
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Player {
    x: usize,
    y: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub enum State {
    Menu,
    Overworld,
    Saying(String, usize),
}

impl Default for State {
    fn default() -> Self {
        Self::Menu
    }
}

pub struct World {
    font_timer: Timer,
    state: State,
    player: Player,
    tilemap: Tilemap,
}

impl Default for World {
    fn default() -> Self {
        Self {
            font_timer: Timer::new(Duration::from_millis(50)),
            state: State::default(),
            player: Player::default(),
            tilemap: load("start.txt"),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_key(&mut self, key: Keycode) {
        match self.state {
            State::Menu => match key {
                Keycode::Return => {
                    self.state = State::Overworld;
                }
                _ => {}
            },
            State::Overworld => {
                if let Keycode::T = key {
                    self.state = State::Saying("Hello, Human!".to_owned(), 0);
                }

                let (x, y) = match key {
                    Keycode::Up | Keycode::W | Keycode::K if self.player.y > 0 => {
                        (self.player.x, self.player.y - 1)
                    }
                    Keycode::Down | Keycode::S | Keycode::J => (self.player.x, self.player.y + 1),
                    Keycode::Left | Keycode::A | Keycode::H if self.player.x > 0 => {
                        (self.player.x - 1, self.player.y)
                    }
                    Keycode::Right | Keycode::D | Keycode::L => (self.player.x + 1, self.player.y),
                    _ => (self.player.x, self.player.y),
                };

                if !self.tilemap.wall_at(x, y) {
                    self.player.x = x;
                    self.player.y = y;
                }
            }
            State::Saying(ref string, ref mut n) => {
                if let Keycode::Return = key {
                    if *n == string.len() {
                        self.state = State::Overworld;
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Saying(ref string, ref mut n) => {
                if *n < string.len() && self.font_timer.finished() {
                    loop {
                        *n += 1;
                        if string.as_bytes().get(*n) != Some(&b' ') { break }
                    }
                    self.font_timer.reset();
                }
            },
            _ => {}
        }
    }

    pub fn draw_menu(&self, screen: &mut Screen<'_>) -> SdlResult<()> {
        screen.draw_text(
            "\x01 Rpg game \x01".to_owned(),
            Screen::WIDTH / 2 - 6,
            Screen::HEIGHT / 2 - 3,
        );
        screen.draw_text(
            "Press enter to start".to_owned(),
            Screen::WIDTH / 2 - 10,
            Screen::HEIGHT / 2,
        );
        Ok(())
    }

    pub fn draw_playing(&mut self, screen: &mut Screen<'_>) -> SdlResult<()> {
        self.tilemap.draw(screen);
        screen.set(self.player.x, self.player.y, Glyph::new(1));
        if let State::Saying(ref string, n) = self.state {
            screen.draw_dialogue(string[0..n].to_owned());
        }
        Ok(())
    }

    pub fn draw(&mut self, screen: &mut Screen<'_>) -> SdlResult<()> {
        match self.state {
            State::Menu => self.draw_menu(screen)?,
            State::Overworld | State::Saying(_, _) => self.draw_playing(screen)?,
        }
        Ok(())
    }
}
