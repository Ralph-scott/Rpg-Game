use crate::*;
use sdl2::keyboard::Keycode;

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
    SayingHello(usize)
}

impl Default for State {
    fn default() -> Self {
        Self::Menu
    }
}

pub struct World {
    state: State,
    player: Player,
    tilemap: Tilemap,
}

impl Default for World {
    fn default() -> Self {
        Self {
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

    pub fn update(&mut self, key: Keycode) {
        let len = "Hello, i am bob the adventurer.".len();
        match self.state {
            State::Menu => match key {
                Keycode::Return => {
                    self.state = State::Overworld;
                }
                _ => {}
            },
            State::Overworld => {
                if let Keycode::T = key {
                    self.state = State::SayingHello(0);
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
            },
            State::SayingHello(l) => {
                if let Keycode::Return = key {
                    if l == len {
                        self.state = State::Overworld;
                    }
                }
            }
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
        if let State::SayingHello(n) = self.state {
            screen.draw_dialogue("Hello, i am bob the adventurer."[0..n].to_owned());
            if n < "Hello, i am bob the adventurer.".len() {
                self.state = State::SayingHello(n + 1);
            }
        }
        Ok(())
    }

    pub fn draw(&mut self, screen: &mut Screen<'_>) -> SdlResult<()> {
        match self.state {
            State::Menu => self.draw_menu(screen)?,
            State::Overworld | State::SayingHello(_) => self.draw_playing(screen)?,
        }
        Ok(())
    }
}
