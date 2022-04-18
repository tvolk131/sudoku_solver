use super::puzzle::{SudokuPuzzle, SudokuPuzzleCell};

pub fn attempt_to_solve(puzzle: &mut SudokuPuzzle) {
    for _ in 1..100 {
        reduce_by_row(puzzle);
        reduce_by_column(puzzle);
        reduce_by_block(puzzle);
    }
}

fn reduce_by_row(puzzle: &mut SudokuPuzzle) {
    for i in 0..puzzle.get_height() {
        reduce_group(puzzle.get_row_mut(i));
    }
}

fn reduce_by_column(puzzle: &mut SudokuPuzzle) {
    for i in 0..puzzle.get_width() {
        reduce_group(puzzle.get_column_mut(i));
    }
}

fn reduce_by_block(puzzle: &mut SudokuPuzzle) {
    for block in puzzle.get_all_blocks_mut() {
        reduce_group(block);
    }
}

fn reduce_group(group: Vec<&mut SudokuPuzzleCell>) {
    let mut taken_values = Vec::new();

    for cell in &group {
        if let Some(value) = cell.get_value() {
            taken_values.push(value);
        }
    }

    for cell in group {
        if !cell.has_value() {
            for column_value in &taken_values {
                cell.remove_possibility(*column_value);
            }
        }
    }
}
