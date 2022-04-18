mod puzzle;
mod sample_puzzles;

fn main() {
    println!("Welcome to sudoku solver!");
    let puzzle = sample_puzzles::get_puzzle_1();
    println!("{}", puzzle);
}
