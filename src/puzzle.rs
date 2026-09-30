//

use crate::matrix::SquareMatrix;

// only 1 horse
// always two portal per used id
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Horse,
    // Block,
    Pool,
    Fruit,
    Bees,
    Portal { id: usize },
}

type Solution = Vec<(usize, usize)>;
#[derive(Debug, Clone)]
pub struct Puzzle {
    pub board: SquareMatrix<Cell>,
    pub num_blocks: usize,
}

// represents a solution quality
pub enum Quality {
    Escaped {
        leaks: usize,
    },

    Enclosed {
        area: usize,
        bees: usize,
        fruits: usize,
    },
}

impl Puzzle {
    pub fn eval_solution() -> Quality {
        todo!()
    }
}
