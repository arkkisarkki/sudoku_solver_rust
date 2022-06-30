use solver::Solver;

mod solver;
mod sudoku;

pub fn main() {
    let mut run = true;
    while run {
        let sudoku = match Solver::generate(70) {
            Ok(sudoku_) => sudoku_,
            Err(err) => {
                println!("Error generating sudoku: {:?}", err);
                return;
            }
        };
        println!("New sudoku:\n{}", sudoku);

        let mut solver = Solver::new(sudoku);
        match solver.solve() {
            Ok(_) => println!("Solution:\n{}", solver),
            Err(err) => {
                println!("Error solving sudoku: {:?}", err);
                run = false;
            }
        }
    }
}
