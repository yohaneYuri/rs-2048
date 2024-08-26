use std::{io::{self, stdout, Write}, time::{Duration, Instant}};

use crossterm::{cursor, event::{self, read, KeyCode, KeyEvent}, execute, terminal::{self, Clear}, ExecutableCommand};

use crate::{board::Board, logic::*, SIZE};

/// Abstraction of game operation from player  
/// Acts as the application message
enum Operation {
    Slide(Direction),
    Quit,
    Restart
}

/// Direction player choose to slide
enum Direction {
    Up,
    Down,
    Left,
    Right
}

/// Struct containing game states for running
pub struct Game {
    board: Board,
    score: u32,
    out: io::Stdout,
    last_keypress_time: Instant
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            score: 0,
            out: stdout(),
            last_keypress_time: Instant::now()
        }
    }

    /// Starts game loop of the whole game 
    pub fn run(&mut self) -> io::Result<()> {
        self.generate_tiles_randomly();
        self.display()?;
        loop {
            if let Some(op) = self.capture_input()? {
                match op {
                    Operation::Slide(dir) => {
                        // Execute the slide
                        // and then update how many tiles occupied the board
                        // finally randomly generate a tile
                        // which can be 2 or 4 (latter has a very small probability)
                        match dir {
                            Direction::Up => self.slide_up(),
                            Direction::Down => self.slide_down(),
                            Direction::Left => self.slide_left(),
                            Direction::Right => self.slide_right(),
                        }
                        self.update_board_filled();
                        
                        if self.board.get_tiles_filled() as usize != SIZE * SIZE {    
                            self.generate_tiles_randomly();
                            self.board.set_tiles_filled(self.board.get_tiles_filled() + 1);
                        }
                        
                        // Check whether to end the game
                        if self.is_game_over() {
                            break;
                        }

                        // Print the board to terminal
                        self.display()?;
                    },
                    Operation::Quit => break,
                    Operation::Restart => {
                        self.reset();
                        self.display()?;
                    },
                }
            }
        }

        // Clear the whole screen
        // and print score
        execute!(
            self.out,
            Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        println!("Your final score: {}", self.score);

        // Press q to quit the game
        println!("Press Q to quit");
        loop {
            if let Some(op) = self.capture_input()? {
                match op {
                    Operation::Quit => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn slide_left(&mut self) {
        for i in 0..SIZE {
            self.score += self.board.slide_line_left(i);
        }
    }

    fn slide_right(&mut self) {
        for i in 0..SIZE {
            self.board.get_line_mut(i).unwrap().reverse();
            self.score += self.board.slide_line_left(i);
            self.board.get_line_mut(i).unwrap().reverse();
        }
    }

    fn slide_up(&mut self) {
        let board = &mut self.board;
        board.transpose();
        for i in 0..SIZE {
            self.score += board.slide_line_left(i);
        }
        board.transpose();
    }

    fn slide_down(&mut self) {
        let board = &mut self.board;
        board.transpose();
        for i in 0..SIZE {
            board.get_line_mut(i).unwrap().reverse();
            self.score += board.slide_line_left(i);
            board.get_line_mut(i).unwrap().reverse();
        }
        board.transpose();
    }

    fn generate_tiles_randomly(&mut self) {
        loop {
            let (x, y) = get_random_position();
            if self.board.get_tile_at(x, y).is_none() {
                self.board.set_tile_value_at(x, y, new_tile());
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.board.clear();
        self.score = 0;
        self.generate_tiles_randomly();
    }

    fn display(&mut self) -> io::Result<()>{
        let stdout = &mut self.out;
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide
        )?;

        for line in self.board.get_tiles() {
            for &tile in line {
                if tile != 0 {
                    print!("{:^5}|", tile);
                } else {
                    print!("     |");
                }
            }
            println!();
        }
        print!("Current score: {}", self.score);

        stdout.execute(cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }

    /// Capture user input and analyze  
    /// transform input to an operation
    fn capture_input(&mut self) -> Result<Option<Operation>, io::Error> {
        const DEBOUNCE_DURATION: Duration = Duration::from_millis(250);

        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(KeyEvent {code, modifiers: _, kind: _, state: _}) = read()? {
                let now = Instant::now();
                if now.duration_since(self.last_keypress_time) < DEBOUNCE_DURATION {
                    return Ok(None);
                }

                self.last_keypress_time = now;
                return match code {
                    KeyCode::Char('q') => Ok(Some(Operation::Quit)),
                    KeyCode::Char('r') => Ok(Some(Operation::Restart)),
                    KeyCode::Up => Ok(Some(Operation::Slide(Direction::Up))),
                    KeyCode::Down => Ok(Some(Operation::Slide(Direction::Down))),
                    KeyCode::Left => Ok(Some(Operation::Slide(Direction::Left))),
                    KeyCode::Right => Ok(Some(Operation::Slide(Direction::Right))),
                    _ => Ok(None)
                }
            }
        }

        Ok(None)
    }

    fn is_game_over(&mut self) -> bool {
        self.board.get_tiles_filled() as usize == SIZE * SIZE &&
            !self.board.has_valid_slides()
    }

    fn update_board_filled(&mut self) {
        let mut filled = 0;
        for line in self.board.get_tiles() {
            let mut line = line.clone();
            line.retain(|&v| v != 0);
            filled += line.len();
        }
        self.board.set_tiles_filled(filled as u32);
    }
}