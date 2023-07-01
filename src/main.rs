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
}
