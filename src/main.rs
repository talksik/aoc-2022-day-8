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

fn main() {
    println!("Hello, world!");

    let input = read_input();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {}
}
