use crate::{logic::check_position, SIZE};

/// Abstraction of game board
pub struct Board {
    tiles: Vec<Vec<u32>>,
    filled: u32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![0; SIZE]; SIZE],
            filled: 0
        }
    }

    pub fn get_tiles(&self) -> &Vec<Vec<u32>> {
        &self.tiles
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<u32> {
        check_position(x, y).then_some(self.tiles[x][y]).filter(|&v| v != 0)
    }
    

    pub fn set_tile_value_at(&mut self, x: usize, y: usize, val: u32) {
        if self.filled as usize == SIZE * SIZE {
            return;
        }
        if check_position(x, y) {
            if self.tiles[x][y] != 0 {
                self.filled += 1;
            }
            self.tiles[x][y] = val;
        }
    }

    pub fn clear(&mut self) {
        self.filled = 0;
        for line in self.tiles.iter_mut() {
            line.fill(0);
        }
    }

    pub fn get_tiles_filled(&self) -> u32 {
        self.filled
    }

    pub fn set_tiles_filled(&mut self, val: u32) {
        self.filled = val;
    }

    pub fn has_valid_slides(&self) -> bool {
        let mut board = self.tiles.clone();
        for line in board.iter_mut() {
            line.retain(|&v| v != 0);
            if line.len() != SIZE {
                return true;
            }
        }

        for i in 0..SIZE {
            for j in 0..SIZE {
                let current =  board[i][j];
                if (i > 0 && current == board[i - 1][j]) ||
                    (i < SIZE - 1 && current == board[i + 1][j]) ||
                    (j > 0 && current == board[i][j - 1]) ||
                    (j < SIZE - 1 && current == board[i][j + 1]) {
                    return true;
                }
            }
        }
        false
    }

    // Transpose the matrix
    // after transposing, reversing, all direction of slide can be converted to sliding left
    pub fn transpose(&mut self) {
        let size = self.tiles.len();
        for i in 0..size {
            for j in (i + 1)..size {
                let temp = self.tiles[i][j];
                self.tiles[i][j] = self.tiles[j][i];
                self.tiles[j][i] = temp;
            }
        }
    }

    pub fn slide_line_left(&mut self, idx: usize) -> u32 {
        match self.tiles.get_mut(idx) {
            Some(line) => {
                line.retain(|&v| v != 0);
                let mut score_increacement = 0;
            
                if !line.is_empty() {
                    for i in 0..(line.len() - 1) {
                        if line[i] == line[i + 1] {
                            line[i] *= 2;
                            line[i + 1] = 0;
                            score_increacement += line[i];
                        }
                    }

                    line.retain(|&v| v != 0);
                }

                while line.len() < SIZE {
                    line.push(0);
                }

                score_increacement
            },
            None => 0
        }
    }

    pub fn get_line_mut(&mut self, idx: usize) -> Option<&mut Vec<u32>> {
        self.tiles.get_mut(idx)
    }
}