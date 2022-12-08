use nalgebra::DMatrix;
use take_until::TakeUntilExt;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Forest {
    tree_heights: DMatrix<u8>,
}

impl Forest {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let columns = input.lines().next().unwrap().len();
        let tree_heights: DMatrix<u8> = DMatrix::from_row_iterator(
            rows,
            columns,
            input
                .chars()
                .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok())),
        );

        Self { tree_heights }
    }

    pub fn visible_trees(&self) -> usize {
        let mut visible_matix: DMatrix<bool> =
            DMatrix::from_element(self.tree_heights.nrows(), self.tree_heights.ncols(), false);

        for cell in visible_matix.row_mut(0).iter_mut() {
            *cell = true;
        }

        for cell in visible_matix.row_mut(visible_matix.nrows() - 1).iter_mut() {
            *cell = true;
        }

        for cell in visible_matix.column_mut(0).iter_mut() {
            *cell = true;
        }

        for cell in visible_matix
            .column_mut(visible_matix.ncols() - 1)
            .iter_mut()
        {
            *cell = true;
        }

        for (row_id, row) in self.tree_heights.row_iter().enumerate() {
            let mut max_height = row[0];
            for (col_id, cell) in row.iter().enumerate().skip(1) {
                if *cell > max_height {
                    visible_matix[(row_id, col_id)] |= true;
                    max_height = *cell;
                }
            }

            max_height = row[row.len() - 1];
            for (col_id, cell) in row.iter().enumerate().rev().skip(1) {
                if *cell > max_height {
                    visible_matix[(row_id, col_id)] |= true;
                    max_height = *cell;
                }
            }
        }

        for (col_id, col) in self.tree_heights.column_iter().enumerate() {
            let mut max_height = col[0];
            for (row_id, cell) in col.iter().enumerate().skip(1) {
                if *cell > max_height {
                    visible_matix[(row_id, col_id)] |= true;
                    max_height = *cell;
                }
            }

            max_height = col[col.len() - 1];
            for (row_id, cell) in col.iter().enumerate().rev().skip(1) {
                if *cell > max_height {
                    visible_matix[(row_id, col_id)] |= true;
                    max_height = *cell;
                }
            }
        }

        visible_matix.iter().filter(|&&v| v).count()
    }

    pub fn scenic_score(&self) -> usize {
        let mut max_scenic: usize = 0;

        for (cell_id, cell) in self.tree_heights.iter().enumerate() {
            let (row, col) = self.tree_heights.vector_to_matrix_index(cell_id);

            let left_view: usize = self
                .tree_heights
                .row(row)
                .iter()
                .take(col)
                .rev()
                .take_until(|&h| *h >= *cell)
                .count();

            let right_view: usize = self
                .tree_heights
                .row(row)
                .iter()
                .skip(col + 1)
                .take_until(|&h| *h >= *cell)
                .count();

            let top_view: usize = self
                .tree_heights
                .column(col)
                .iter()
                .take(row)
                .rev()
                .take_until(|&h| *h >= *cell)
                .count();

            let bottom_view: usize = self
                .tree_heights
                .column(col)
                .iter()
                .skip(row + 1)
                .take_until(|&h| *h >= *cell)
                .count();

            let scenic_score = left_view * right_view * top_view * bottom_view;

            max_scenic = max_scenic.max(scenic_score);
        }

        max_scenic
    }
}

pub fn part1(input: &str) {
    let forest = Forest::new(input);
    let visible_trees = forest.visible_trees();
    println!("Visible trees: {}", visible_trees);
}

pub fn part2(input: &str) {
    let forest = Forest::new(input);
    let maximum_scenic_score = forest.scenic_score();
    println!("Maximum scenic score: {}", maximum_scenic_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STRING: &str = "30373
    25512
    65332
    33549
    35390";

    #[test]
    fn simple_visible() {
        let forest = Forest::new(INPUT_STRING);
        let visible_trees = forest.visible_trees();

        assert_eq!(visible_trees, 21);
    }

    #[test]
    fn simple_scenic() {
        let forest = Forest::new(INPUT_STRING);
        let maximum_scenic_score = forest.scenic_score();

        assert_eq!(maximum_scenic_score, 8);
    }
}
