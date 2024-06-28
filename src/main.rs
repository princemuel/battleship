use rand::Rng;
use std::usize;
mod utils;

const BOARD_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    fn can_place_ship(
        &self,
        row: usize,
        col: usize,
        size: usize,
        direction: bool,
    ) -> bool {
        if direction {
            if col + size > BOARD_SIZE {
                return false;
            }
            for num in 0..size {
                if self.grid[row][col + num] != CellState::Empty {
                    return false;
                }
            }
        } else {
            if row + size > BOARD_SIZE {
                return false;
            }
            for num in 0..size {
                if self.grid[row + num][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction = rng.gen::<bool>();

            if self.can_place_ship(row, col, size, direction) {}
        }
    }
}

fn main() {
    println!("Hello, world!");
}
