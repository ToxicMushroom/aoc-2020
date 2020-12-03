use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day3/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let rows: Vec<&str> = contents.lines().collect();
    let height = rows.len();
    exercise1(&rows, height);
    exercise2(&rows, height);
    Ok(())
}

fn exercise1(rows: &Vec<&str>, height: usize) {
    let (x, y) = (3, 1);
    println!("{:?}", get_trees_on_slope(rows, height, &(x, y)));
}

fn get_trees_on_slope(rows: &Vec<&str>, height: usize, slope: &(usize, usize)) -> usize {
    let (x, y) = slope;
    let pattern_width = rows.get(0).unwrap().len();
    let (mut y_pos, mut x_pos) = (0, 0);
    let mut trees: usize = 0;
    while y_pos < height {
        let tree_or_not_tree = rows[y_pos].chars().nth(x_pos).unwrap();
        trees += (tree_or_not_tree == '#') as usize;
        x_pos += x;
        y_pos += y;
        if x_pos >= pattern_width {
            x_pos -= pattern_width
        }
    }
    trees
}

fn exercise2(rows: &Vec<&str>, height: usize) {
    let tuples: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut trees_multiplied = 1;
    for tuple in tuples.iter() {
        trees_multiplied *= get_trees_on_slope(rows, height, tuple);
    }
    println!("{:?}", trees_multiplied);
}