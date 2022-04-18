pub struct SudokuPuzzle {
    block_rows: Vec<Vec<SudokuPuzzleBlock>>,
}

impl SudokuPuzzle {
    pub fn new(block_rows: Vec<Vec<SudokuPuzzleBlock>>) -> Self {
        Self::assert_square_cell_shape(&block_rows);
        Self { block_rows }
    }

    pub fn new_basic_9_by_9(matrix: [[usize; 9]; 9]) -> Self {
        let mut block_rows = Vec::new();

        for i in 0..3 {
            let mut row = Vec::new();

            for j in 0..3 {
                let mut block = SudokuPuzzleBlock::new(3, 3);

                block.set_cell_value_if_nonzero(0, 0, matrix[i * 3][j * 3]);
                block.set_cell_value_if_nonzero(0, 1, matrix[i * 3][j * 3 + 1]);
                block.set_cell_value_if_nonzero(0, 2, matrix[i * 3][j * 3 + 2]);
                block.set_cell_value_if_nonzero(1, 0, matrix[i * 3 + 1][j * 3]);
                block.set_cell_value_if_nonzero(1, 1, matrix[i * 3 + 1][j * 3 + 1]);
                block.set_cell_value_if_nonzero(1, 2, matrix[i * 3 + 1][j * 3 + 2]);
                block.set_cell_value_if_nonzero(2, 0, matrix[i * 3 + 2][j * 3]);
                block.set_cell_value_if_nonzero(2, 1, matrix[i * 3 + 2][j * 3 + 1]);
                block.set_cell_value_if_nonzero(2, 2, matrix[i * 3 + 2][j * 3 + 2]);

                row.push(block);
            }

            block_rows.push(row);
        }

        Self::new(block_rows)
    }

    /// Asserts that all blocks in a given matrix are the same dimensions, and panics if they are not.
    fn assert_all_blocks_are_same_shape(block_rows: &Vec<Vec<SudokuPuzzleBlock>>) {
        let mut block_iter = block_rows.iter().flatten();

        let first_block = match block_iter.next() {
            Some(first_block) => first_block,
            None => return,
        };

        let first_block_height = first_block.get_cell_height();
        let first_block_width = first_block.get_cell_width();

        for block in block_rows.iter().flatten() {
            if block.get_cell_height() != first_block_height {
                panic!("Non-matching block height!");
            }

            if block.get_cell_width() != first_block_width {
                panic!("Non-matching block width!");
            }
        }
    }

    /// Asserts that a block matrix is square with respect to the cell count.
    /// That is, it has the same number of cells in each row and column.
    fn assert_square_cell_shape(block_rows: &Vec<Vec<SudokuPuzzleBlock>>) {
        Self::assert_all_blocks_are_same_shape(block_rows);

        let first_block = match block_rows.iter().flatten().next() {
            Some(first_block) => first_block,
            None => return,
        };

        let block_height = first_block.get_cell_height();
        let block_width = first_block.get_cell_width();

        let total_cell_length = block_height * block_rows.len();

        for row in block_rows {
            if row.len() * block_width != total_cell_length {
                panic!("Blocks do not form a square shape!");
            }
        }
    }

    /// Gets the top-left block in the puzzle, or returns None if the puzzle is empty.
    fn get_first_block_or(&self) -> Option<&SudokuPuzzleBlock> {
        self.block_rows.first()?.first()
    }

    fn get_block_height(&self) -> usize {
        match self.get_first_block_or() {
            Some(first_block) => first_block.get_cell_height(),
            None => return 0,
        }
    }

    fn get_block_width(&self) -> usize {
        match self.get_first_block_or() {
            Some(first_block) => first_block.get_cell_width(),
            None => return 0,
        }
    }

    /// Gets the number of cell blocks stacked on top of one another for this puzzle.
    fn get_vertical_block_count(&self) -> usize {
        self.block_rows.len()
    }

    /// Gets the number of cell blocks stacked side-to-side for this puzzle.
    fn get_horizontal_block_count(&self) -> usize {
        match self.block_rows.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }

    fn get_height(&self) -> usize {
        self.get_block_height() * self.get_vertical_block_count()
    }

    fn get_width(&self) -> usize {
        self.get_block_width() * self.get_horizontal_block_count()
    }

    fn get_row(&self, mut row_index: usize) -> Vec<&SudokuPuzzleCell> {
        // Computing this here so we don't have to call this repeatedly.
        let block_height = self.get_block_height();

        let mut block_index = 0;
        while row_index >= block_height {
            block_index += 1;
            row_index -= block_height
        }

        let mut row_cells = Vec::new();

        for horizontal_blocks in self.block_rows.get(block_index) {
            for block in horizontal_blocks {
                row_cells.append(&mut block.get_block_row(row_index));
            }
        }

        row_cells
    }

    fn get_column(&self, mut column_index: usize) -> Vec<&SudokuPuzzleCell> {
        // Computing this here so we don't have to call this repeatedly.
        let block_width = self.get_block_width();

        let mut block_index = 0;
        while column_index > block_width {
            block_index += 1;
            column_index -= block_width
        }

        let mut column_cells = Vec::new();

        for block_row in &self.block_rows {
            if let Some(block) = block_row.get(block_index) {
                column_cells.append(&mut block.get_block_column(column_index));
            }
        }

        column_cells
    }
}

impl std::fmt::Display for SudokuPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_height = self.get_block_height();
        let block_width = self.get_block_width();

        for i in 0..self.get_height() {
            if i % block_height == 0 {
                write!(f, " ")?;
                for i in 0..self.get_width() + self.get_block_width() {
                    write!(f, "-")?;
                }
                writeln!(f, " ")?;
            }

            for (i, cell) in self.get_row(i)
            .iter()
            .enumerate() {
                if i % block_width == 0 {
                    write!(f, "|")?;
                }
                write!(f, "{}", match cell.get_value() {
                    Some(value) => format!("{}", value),
                    None => " ".to_string(),
                })?;
            }
            writeln!(f, "|")?;
        }

        write!(f, " ")?;
        for i in 0..self.get_width() {
            write!(f, "-")?;
        }
        write!(f, " ")?;

        Ok(())
    }
}

pub struct SudokuPuzzleBlock {
    cell_rows: Vec<Vec<SudokuPuzzleCell>>,
}

impl SudokuPuzzleBlock {
    fn new(height: usize, width: usize) -> Self {
        let mut cell_rows = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(SudokuPuzzleCell::new(height * width));
            }
            cell_rows.push(row);
        }

        Self { cell_rows }
    }

    /// Gets the number of cells stacked on top of one another for this block.
    fn get_cell_height(&self) -> usize {
        self.cell_rows.len()
    }

    /// Gets the number of cells stacked side-to-side for this block.
    fn get_cell_width(&self) -> usize {
        match self.cell_rows.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }

    fn get_block_row(&self, row_index: usize) -> Vec<&SudokuPuzzleCell> {
        let row = match self.cell_rows.get(row_index) {
            Some(row) => row,
            None => return Vec::new(),
        };

        let mut borrowed_cells = Vec::new();

        for cell in row {
            borrowed_cells.push(cell);
        }

        borrowed_cells
    }

    fn get_block_column(&self, column_index: usize) -> Vec<&SudokuPuzzleCell> {
        let mut borrowed_cells = Vec::new();

        for row in &self.cell_rows {
            if let Some(foo) = row.get(column_index) {
                borrowed_cells.push(foo);
            }
        }

        borrowed_cells
    }

    fn set_cell_value_if_nonzero(&mut self, row_index: usize, column_index: usize, value: usize) {
        if value != 0 {
            self.set_cell_value(row_index, column_index, value);
        }
    }

    fn set_cell_value(&mut self, row_index: usize, column_index: usize, value: usize) {
        let row = match self.cell_rows.get_mut(row_index) {
            Some(row) => row,
            None => panic!("Cannot set cell value for cell that does not exist!"),
        };

        let cell = match row.get_mut(column_index) {
            Some(cell) => cell,
            None => panic!("Cannot set cell value for cell that does not exist!"),
        };

        cell.set_value(value);
    }
}

struct SudokuPuzzleCell {
    possibilities: Vec<usize>,
}

impl SudokuPuzzleCell {
    fn new(max_value: usize) -> Self {
        let mut possibilities = Vec::new();

        for i in 1..max_value + 1 {
            possibilities.push(i);
        }

        if possibilities.is_empty() {
            panic!("Cannot initialize cell with no possible values!");
        }

        Self { possibilities }
    }

    fn get_value(&self) -> Option<usize> {
        if self.possibilities.len() != 1 {
            return None;
        }

        self.possibilities.first().copied()
    }

    fn set_value(&mut self, value: usize) {
        if !self.contains_possibility(value) {
            panic!("Cannot set value to cell that was already marked as not a possibility!");
        }

        self.possibilities = vec![value];
    }

    fn contains_possibility(&self, possibility: usize) -> bool {
        for cell_possibility in &self.possibilities {
            if cell_possibility == &possibility {
                return true;
            }
        }

        false
    }

    /// Removes a possibility for this cell.
    /// Panics if the cell already has only one possible value.
    /// Does nothing if the possibility being removed was already removed.
    fn remove_possibility(&mut self, possibility: usize) {
        if self.get_value().is_some() {
            panic!("Cannot remove possibility from cell that already has a found value!");
        }

        self.possibilities = self
            .possibilities
            .iter()
            .cloned()
            .filter(|current_possibility| current_possibility != &possibility)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sudoku_puzzle_cell_correct_initial_values() {
        let mut cell = SudokuPuzzleCell::new(1);
        assert_eq!(cell.possibilities, vec![1]);

        cell = SudokuPuzzleCell::new(9);
        assert_eq!(cell.possibilities, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    #[should_panic]
    fn sudoku_puzzle_cell_panics_on_empty_init() {
        SudokuPuzzleCell::new(0);
    }

    #[test]
    fn sudoku_puzzle_get_row() {
        let puzzle = super::super::sample_puzzles::get_puzzle_1();

        assert_eq!(
            puzzle
                .get_row(0)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![6, 0, 5, 4, 0, 0, 3, 0, 2]
        );

        assert_eq!(
            puzzle
                .get_row(1)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![7, 3, 4, 0, 6, 0, 0, 5, 8]
        );

        assert_eq!(
            puzzle
                .get_row(2)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![0, 1, 0, 5, 3, 0, 0, 0, 0]
        );

        assert_eq!(
            puzzle
                .get_row(3)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![0, 4, 2, 6, 0, 7, 1, 9, 5]
        );

        assert_eq!(
            puzzle
                .get_row(4)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![0, 9, 7, 0, 0, 4, 0, 6, 0]
        );

        assert_eq!(
            puzzle
                .get_row(5)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![0, 0, 0, 0, 1, 3, 0, 0, 7]
        );

        assert_eq!(
            puzzle
                .get_row(6)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![9, 0, 6, 3, 0, 5, 0, 0, 0]
        );

        assert_eq!(
            puzzle
                .get_row(7)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![4, 7, 0, 0, 9, 1, 0, 2, 0]
        );

        assert_eq!(
            puzzle
                .get_row(8)
                .iter()
                .map(|cell| cell.get_value().unwrap_or(0))
                .collect::<Vec<usize>>(),
            vec![0, 0, 0, 0, 2, 0, 0, 0, 0]
        );
    }
}
