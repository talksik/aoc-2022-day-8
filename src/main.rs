// reads the input and puts into a matrix.
// each inner vector is a row of the matrix
fn read_input() -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let mut trees = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for tree in line.chars() {
            // parse char to u32
            let tree_height = tree.to_digit(10).unwrap();
            row.push(tree_height);
        }

        trees.push(row);
    }

    trees
}

fn count_perimeter(num_rows: usize, num_cols: usize) -> u32 {
    ((num_rows * 2) + (num_cols * 2) - 4) as u32
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    max_right: u32,
    max_left: u32,
    max_up: u32,
    max_down: u32,
    is_perimeter: bool,
}

impl Tree {
    pub fn is_visible_somewhere(&self) -> bool {
        if self.is_perimeter {
            return true;
        }

        self.height > self.max_right
            || self.height > self.max_left
            || self.height > self.max_up
            || self.height > self.max_down
    }
}

// takes in grid of trees and returns a grid of trees
fn process_trees(trees: &[&[u32]]) -> Vec<Tree> {
    let mut processed_trees = vec![];

    for (row_index, row) in trees.iter().enumerate() {
        for (col_index, tree_height) in row.iter().enumerate() {
            if col_index == 0
                || row_index == 0
                || col_index == row.len() - 1
                || row_index == trees.len() - 1
            {
                let processed_tree = Tree {
                    height: *tree_height,
                    max_right: 0,
                    max_left: 0,
                    max_up: 0,
                    max_down: 0,
                    is_perimeter: true,
                };

                processed_trees.push(processed_tree);
                continue;
            }

            // get max of all trees to the right
            let max_right = row[(col_index + 1)..].iter().max().unwrap();
            let max_left = row[..col_index].iter().max().unwrap();
            let max_up = trees[..row_index]
                .iter()
                .map(|row| row[col_index])
                .max()
                .unwrap();
            let max_down = trees[(row_index + 1)..]
                .iter()
                .map(|row| row[col_index])
                .max()
                .unwrap();

            let processed_tree = Tree {
                height: *tree_height,
                max_right: *max_right,
                max_left: *max_left,
                max_up,
                max_down,
                is_perimeter: false,
            };

            println!("processed tree: {:?}", processed_tree);

            processed_trees.push(processed_tree);
        }
    }

    processed_trees
}

fn count_visible_trees(trees: Vec<Tree>) -> u32 {
    let mut visible_trees = 0;

    for tree in trees {
        if tree.is_visible_somewhere() {
            visible_trees += 1;
        }
    }

    visible_trees
}

fn main() {
    println!("Hello, world!");

    let trees = read_input();

    println!(
        "how many rows {:?}, how many columns {:?}",
        trees.len(),
        trees[0].len()
    );

    println!(
        "perimeter is {:?}",
        count_perimeter(trees.len(), trees[0].len())
    );

    let trees: Vec<&[u32]> = trees.iter().map(|row| row.as_slice()).collect();
    let processed_trees = process_trees(&trees);

    let visible_trees = count_visible_trees(processed_trees);

    println!("visible trees: {:?}", visible_trees);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}

    #[test]
    fn test_count_perimeter() {
        assert_eq!(count_perimeter(3, 3), 8_u32);
        assert_eq!(count_perimeter(5, 5), 16_u32);
    }

    #[test]
    fn test_part_one() {
        // mimic the above grid into a vec of vecs
        let trees = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        let trees: Vec<&[u32]> = trees.iter().map(|row| row.as_slice()).collect();
        let processed_trees = process_trees(&trees);
        assert_eq!(count_visible_trees(processed_trees), 21);
    }
}
