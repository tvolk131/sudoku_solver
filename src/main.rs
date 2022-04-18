mod puzzle;
mod sample_puzzles;
mod solver;

fn main() {
    println!("Welcome to sudoku solver!");
    let mut puzzle = sample_puzzles::get_puzzle_1();
    println!("{}", puzzle);
    solver::attempt_to_solve(&mut puzzle);
    println!("{}", puzzle);
}
