use super::puzzle::SudokuPuzzle;

pub fn get_puzzle_1() -> SudokuPuzzle {
    SudokuPuzzle::new_basic_9_by_9([
        [6, 0, 5, 4, 0, 0, 3, 0, 2],
        [7, 3, 4, 0, 6, 0, 0, 5, 8],
        [0, 1, 0, 5, 3, 0, 0, 0, 0],
        [0, 4, 2, 6, 0, 7, 1, 9, 5],
        [0, 9, 7, 0, 0, 4, 0, 6, 0],
        [0, 0, 0, 0, 1, 3, 0, 0, 7],
        [9, 0, 6, 3, 0, 5, 0, 0, 0],
        [4, 7, 0, 0, 9, 1, 0, 2, 0],
        [0, 0, 0, 0, 2, 0, 0, 0, 0],
    ])
}