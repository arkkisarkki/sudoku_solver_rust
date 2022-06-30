use crate::{
    check, coords,
    sudoku::{Coordinates, Grid, Sudoku, SudokuError},
};
use rand::Rng;
use std::{collections::HashSet, fmt::Display};

macro_rules! all_possible {
    () => {
        HashSet::from([1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8])
    };
}

#[derive(Debug)]
pub struct Solver {
    sudoku: Sudoku,
    last_secure_state: Grid,
    last_secure_state_set: bool,
}

#[derive(Debug)]
pub enum SolverError {
    NoPossibilities,
    SudokuError(SudokuError),
}

type Neighbors = HashSet<Coordinates>;

trait FromCoordinates<T> {
    fn from_coordinates(row: usize, column: usize) -> T;
}

impl FromCoordinates<Neighbors> for Neighbors {
    fn from_coordinates(row: usize, column: usize) -> Neighbors {
        let mut retval = Neighbors::new();

        for i in 0..9 {
            // Row
            retval.insert(Coordinates { row, column: i });
            // Column
            retval.insert(Coordinates { row: i, column });
        }
        // Block
        let block_row = row / 3;
        let block_column = column / 3;
        for row_in_b in 0..3 {
            for column_in_b in 0..3 {
                retval.insert(Coordinates {
                    row: block_row * 3 + row_in_b,
                    column: block_column * 3 + column_in_b,
                });
            }
        }

        retval.remove(&Coordinates { row, column });

        retval
    }
}

impl Solver {
    pub fn new(sudoku: Sudoku) -> Solver {
        let last_secure_state = sudoku.squares.clone();
        Solver {
            sudoku,
            last_secure_state,
            last_secure_state_set: false,
        }
    }

    pub fn get_possible(&self, row: usize, column: usize) -> Result<HashSet<u8>, SudokuError> {
        check!(coords row, column);

        let mut retval: HashSet<u8> = all_possible!();

        let row_values = self.sudoku.get_row(row)?;
        let column_values = self.sudoku.get_column(column)?;
        let block_values = self.sudoku.get_block(row / 3, column / 3)?;

        for i in 0..9 {
            retval.remove(&row_values[i]);
            retval.remove(&column_values[i]);
            retval.remove(&block_values[i]);
        }

        Ok(retval)
    }

    fn set_last_secure_state(&mut self) {
        self.last_secure_state = self.sudoku.squares.clone();
        self.last_secure_state_set = true;
    }

    fn step(&mut self) -> Result<(), SolverError> {
        let mut changed = false;
        let mut certain = true;
        let mut lowest_possibilities = all_possible!();
        let mut lowest_possible_coords = Coordinates { column: 0, row: 0 };

        for row in 0usize..9 {
            for column in 0usize..9 {
                if !self
                    .sudoku
                    .is_set(row, column)
                    .map_err(|err| SolverError::SudokuError(err))?
                {
                    let mut possibilities = self
                        .get_possible(row, column)
                        .map_err(|err| SolverError::SudokuError(err))?;

                    if possibilities.is_empty() {
                        certain = false;
                        while possibilities.is_empty() {
                            let neighbors = Neighbors::from_coordinates(row, column);
                            let mut possible_resets = neighbors.clone();
                            for possible_reset in neighbors {
                                if self.last_secure_state
                                    [coords!(possible_reset.row, possible_reset.column)]
                                    != 0
                                {
                                    possible_resets.remove(&possible_reset);
                                }
                            }

                            let mut rng = rand::thread_rng();
                            let len = possible_resets.len();
                            let reset = &possible_resets.into_iter().collect::<Vec<Coordinates>>()
                                [rng.gen_range(0..len)];

                            self.sudoku
                                .set(reset.row, reset.column, 0)
                                .map_err(|err| SolverError::SudokuError(err))?;

                            possibilities = self
                                .get_possible(row, column)
                                .map_err(|err| SolverError::SudokuError(err))?;
                        }
                    }

                    if possibilities.len() <= lowest_possibilities.len() {
                        certain = false;
                        lowest_possibilities = possibilities.clone();
                        lowest_possible_coords.row = row;
                        lowest_possible_coords.column = column;
                    }

                    if possibilities.len() == 1 {
                        changed = true;
                        self.sudoku
                            .set(
                                row,
                                column,
                                *possibilities
                                    .iter()
                                    .next()
                                    .ok_or(SolverError::NoPossibilities)?,
                            )
                            .map_err(|err| SolverError::SudokuError(err))?;
                    }
                }
            }
        }

        if !changed {
            let lowest_vec: Vec<u8> = lowest_possibilities.into_iter().collect();
            let mut rng = rand::thread_rng();
            let lowest = lowest_vec[rng.gen_range(0..lowest_vec.len())];
            self.sudoku
                .set(
                    lowest_possible_coords.row,
                    lowest_possible_coords.column,
                    lowest,
                )
                .map_err(|err| SolverError::SudokuError(err))?;
        } else if !self.last_secure_state_set & certain {
            self.set_last_secure_state();
        }

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SolverError> {
        while self.sudoku.set_count < 9 * 9 {
            self.step()?;
        }
        Ok(())
    }

    pub fn generate(difficulty: u8) -> Result<Sudoku, SolverError> {
        let sudoku = Sudoku::new_empty();
        let mut solver = Solver::new(sudoku);
        solver.solve()?;
        let mut rng = rand::thread_rng();
        for i in 0..9 * 9 {
            if rng.gen_range(0..100) < difficulty {
                solver.sudoku.squares[i] = 0;
            }
        }

        Ok(Sudoku::new_from_state(solver.sudoku.squares))
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.sudoku.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        solver::Solver,
        sudoku::{Sudoku},
    };
    use std::collections::HashSet;

    use super::{FromCoordinates, Neighbors};

    #[test]
    fn test_possible() {
        let mut solver = Solver::new(Sudoku::new_empty());
        assert_eq!(all_possible!(), solver.get_possible(1, 1).unwrap());
        solver.sudoku.set(0, 1, 1).unwrap();
        assert_eq!(
            HashSet::from([2, 3, 4, 5, 6, 7, 8, 9]),
            solver.get_possible(1, 1).unwrap()
        );
    }

    #[test]
    fn test_possible_resets() {
        let resets = Neighbors::from_coordinates(1, 2);
        println!("{:#?}", resets);
    }

    #[test]
    fn test_generate() {
        let sudoku = Solver::generate(50).unwrap();
        println!("{}", sudoku);
    }
}
