use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game::Game;

mod game;
mod board;
mod logic;

const SIZE: usize = 4;

fn main() -> io::Result<()> {
    let mut game = Game::new();

    enable_raw_mode()?;

    game.run()?;

    disable_raw_mode()?;

    Ok(())
}
