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
        // for incomplete solutions
        bees: usize,
        fruits: usize,
    },

    Enclosed {
        area: usize,
        bees: usize,
        fruits: usize,
    },
}

use std::collections::VecDeque;
impl Puzzle {
    pub fn eval_solution() -> Quality {
        todo!()
    }

    pub fn leaks(&self, s: Solution) -> usize {
        let mut queue = VecDeque<_>::new();
        for (i, j) in self.borders() {
            queue.push_back((i, j));
            
        }

        todo!()
    }

    fn borders(&self) -> Vec<(usize, usize)> {
        todo!()
    }
}
