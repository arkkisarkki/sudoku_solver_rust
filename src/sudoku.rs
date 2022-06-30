use std::fmt::Display;

pub type Grid = [u8; 9 * 9];
pub type Row = [u8; 9];
pub type Column = [u8; 9];
pub type Block = [u8; 9];

#[macro_export]
macro_rules! coords {
    ($row: expr, $column: expr) => {
        $row * 9 + $column
    };
}

#[macro_export]
macro_rules! check {
    (row $row: expr) => {
        if $row >= 9 {
            return Err(SudokuError::BadRow($row));
        }
    };
    (column $column: expr) => {
        if $column >= 9 {
            return Err(SudokuError::BadColumn($column));
        }
    };
    (coords $row: expr, $column: expr) => {
        if $row >= 9 || $column >= 9 {
            return Err(SudokuError::BadCoordinates($row, $column));
        }
    };
    (value $value: expr) => {
        if $value > 9 {
            return Err(SudokuError::BadValue($value));
        }
    };
}

#[derive(Debug)]
pub struct Sudoku {
    pub squares: Grid,
    pub set_count: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinates {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum SudokuError {
    BadRow(usize),
    BadColumn(usize),
    BadCoordinates(usize, usize),
    BadValue(u8),
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.squares.iter().enumerate() {
            if i != 0 {
                if i % 27 == 0 {
                    write!(f, "\n----------------------\n")?;
                } else if i % 9 == 0 {
                    write!(f, "\n")?;
                } else if i % 3 == 0 {
                    write!(f, "| ")?;
                }
            }
            if *value != 0 {
                write!(f, "{} ", *value)?;
            } else {
                write!(f, "  ")?;
            }
        }
        Ok(())
    }
}

impl Sudoku {
    pub fn new_from_state(state: Grid) -> Sudoku {
        let mut retval = Sudoku::new_empty();
        for (i, value) in state.into_iter().enumerate() {
            retval.squares[i] = value;
            if value != 0 {
                retval.set_count += 1;
            }
        }
        retval
    }

    pub fn new_empty() -> Sudoku {
        Sudoku {
            squares: [0; 9 * 9],
            set_count: 0,
        }
    }

    pub fn get_row(&self, row: usize) -> Result<Row, SudokuError> {
        check!(row row);

        let mut retval = [0u8; 9];
        for (i, value) in self.squares[row * 9..row * 9 + 9].iter().enumerate() {
            retval[i] = *value;
        }
        Ok(retval)
    }

    pub fn get_column(&self, column: usize) -> Result<Column, SudokuError> {
        check!(column column);

        let mut retval = [0u8; 9];
        for (i, value) in self.squares[column..].iter().step_by(9).enumerate() {
            retval[i] = *value;
        }
        Ok(retval)
    }

    pub fn get_block(&self, block_row: usize, block_column: usize) -> Result<Block, SudokuError> {
        if block_row >= 3 || block_column >= 3 {
            return Err(SudokuError::BadCoordinates(block_row, block_column));
        }
        let mut retval = [0u8; 9];
        for i in 0..9 {
            retval[i] = self.squares[coords!((block_row * 3 + i / 3), (block_column * 3 + i % 3))]
        }
        Ok(retval)
    }

    pub fn set(&mut self, row: usize, column: usize, value: u8) -> Result<(), SudokuError> {
        check!(coords row, column);
        check!(value value);

        if self.squares[coords!(row, column)] == 0 {
            if value != 0 {
                self.set_count += 1;
            }
        } else {
            if value == 0 {
                self.set_count -= 1;
            }
        }

        self.squares[coords!(row, column)] = value;

        Ok(())
    }

    pub fn is_set(&self, row: usize, column: usize) -> Result<bool, SudokuError> {
        check!(coords row, column);
        Ok(self.squares[coords!(row, column)] != 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{Sudoku, SudokuError};

    #[test]
    fn test_fmt() {
        let sudoku = Sudoku {
            squares: [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1,
                2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2,
                3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            ],
            set_count: 0,
        };
        println!("{}", sudoku);
    }

    #[test]
    fn test_get_row() {
        let sudoku = Sudoku::new_from_state([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
        ]);

        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], sudoku.get_row(0).unwrap());
        assert_eq!(
            [10, 11, 12, 13, 14, 15, 16, 17, 18],
            sudoku.get_row(1).unwrap()
        );
        assert_eq!(
            [19, 20, 21, 22, 23, 24, 25, 26, 27],
            sudoku.get_row(2).unwrap()
        );
        assert_eq!(
            [28, 29, 30, 31, 32, 33, 34, 35, 36],
            sudoku.get_row(3).unwrap()
        );
        assert_eq!(
            [37, 38, 39, 40, 41, 42, 43, 44, 45],
            sudoku.get_row(4).unwrap()
        );
        assert_eq!(
            [46, 47, 48, 49, 50, 51, 52, 53, 54],
            sudoku.get_row(5).unwrap()
        );
        assert_eq!(
            [55, 56, 57, 58, 59, 60, 61, 62, 63],
            sudoku.get_row(6).unwrap()
        );
        assert_eq!(
            [64, 65, 66, 67, 68, 69, 70, 71, 72],
            sudoku.get_row(7).unwrap()
        );
        assert_eq!(
            [73, 74, 75, 76, 77, 78, 79, 80, 81],
            sudoku.get_row(8).unwrap()
        );
    }

    #[test]
    fn test_get_column() {
        let sudoku = Sudoku::new_from_state([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
        ]);

        assert_eq!(
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            sudoku.get_column(0).unwrap()
        );
        assert_eq!(
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            sudoku.get_column(1).unwrap()
        );
        assert_eq!(
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            sudoku.get_column(2).unwrap()
        );
        assert_eq!(
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            sudoku.get_column(3).unwrap()
        );
        assert_eq!(
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            sudoku.get_column(4).unwrap()
        );
        assert_eq!(
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            sudoku.get_column(5).unwrap()
        );
        assert_eq!(
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            sudoku.get_column(6).unwrap()
        );
        assert_eq!(
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
            sudoku.get_column(7).unwrap()
        );
        assert_eq!(
            [9, 18, 27, 36, 45, 54, 63, 72, 81],
            sudoku.get_column(8).unwrap()
        );
    }

    #[test]
    fn test_get_block() {
        let sudoku = Sudoku::new_from_state([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
        ]);

        assert_eq!(
            [1, 2, 3, 10, 11, 12, 19, 20, 21],
            sudoku.get_block(0, 0).unwrap()
        );
        assert_eq!(
            [28, 29, 30, 37, 38, 39, 46, 47, 48,],
            sudoku.get_block(1, 0).unwrap()
        );
        assert_eq!(
            [55, 56, 57, 64, 65, 66, 73, 74, 75,],
            sudoku.get_block(2, 0).unwrap()
        );
        assert_eq!(
            [4, 5, 6, 13, 14, 15, 22, 23, 24],
            sudoku.get_block(0, 1).unwrap()
        );
        assert_eq!(
            [31, 32, 33, 40, 41, 42, 49, 50, 51,],
            sudoku.get_block(1, 1).unwrap()
        );
        assert_eq!(
            [58, 59, 60, 67, 68, 69, 76, 77, 78],
            sudoku.get_block(2, 1).unwrap()
        );
        assert_eq!(
            [7, 8, 9, 16, 17, 18, 25, 26, 27],
            sudoku.get_block(0, 2).unwrap()
        );
        assert_eq!(
            [34, 35, 36, 43, 44, 45, 52, 53, 54],
            sudoku.get_block(1, 2).unwrap()
        );
        assert_eq!(
            [61, 62, 63, 70, 71, 72, 79, 80, 81],
            sudoku.get_block(2, 2).unwrap()
        );
    }

    #[test]
    fn test_bad_row() {
        let sudoku = Sudoku::new_from_state([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
        ]);

        assert_eq!(SudokuError::BadRow(10), sudoku.get_row(10).unwrap_err());
    }

    #[test]
    fn test_set() {
        let mut sudoku = Sudoku::new_empty();

        sudoku.set(2, 2, 5).unwrap();
        assert!(sudoku.is_set(2, 2).unwrap());
        assert_eq!(1, sudoku.set_count);

        sudoku.set(2, 2, 7).unwrap();
        assert!(sudoku.is_set(2, 2).unwrap());
        assert_eq!(1, sudoku.set_count);

        sudoku.set(2, 2, 0).unwrap();
        assert!(!sudoku.is_set(2, 2).unwrap());
        assert_eq!(0, sudoku.set_count);

        sudoku.set(2, 3, 0).unwrap();
        assert!(!sudoku.is_set(2, 3).unwrap());
        assert_eq!(0, sudoku.set_count);
    }
}
