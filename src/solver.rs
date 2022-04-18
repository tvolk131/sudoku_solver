use super::puzzle::SudokuPuzzle;

pub fn attempt_to_solve(puzzle: &mut SudokuPuzzle) {
    for _ in 1..1000 {
        reduce_by_row(puzzle);
        reduce_by_column(puzzle);
    }
}

fn reduce_by_row(puzzle: &mut SudokuPuzzle) {
    for i in 0..puzzle.get_height() {
        let row = puzzle.get_row_mut(i);
        let mut row_values = Vec::new();

        for cell in &row {
            if let Some(value) = cell.get_value() {
                row_values.push(value);
            }
        }

        for cell in row {
            if !cell.has_value() {
                for row_value in &row_values {
                    cell.remove_possibility(*row_value);
                }
            }
        }
    }
}

fn reduce_by_column(puzzle: &mut SudokuPuzzle) {
    for i in 0..puzzle.get_width() {
        let column = puzzle.get_column_mut(i);
        let mut column_values = Vec::new();

        for cell in &column {
            if let Some(value) = cell.get_value() {
                column_values.push(value);
            }
        }

        for cell in column {
            if !cell.has_value() {
                for column_value in &column_values {
                    cell.remove_possibility(*column_value);
                }
            }
        }
    }
}
