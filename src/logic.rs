use rand::Rng;

use crate::SIZE;

pub fn check_position(x: usize, y:usize) -> bool {
    x < SIZE && y < SIZE
}

pub fn get_random_position() -> (usize, usize) {
    let ub = SIZE * SIZE;
    let idx: usize = rand::thread_rng().gen_range(0..ub);
    (idx / SIZE, idx % SIZE)
}

pub fn new_tile() -> u32 {
    let sample = rand::thread_rng().gen_range(0..100);
    if sample > 90 {
        4
    } else {
        2
    }
}