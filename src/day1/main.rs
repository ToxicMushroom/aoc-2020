use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day1/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    exercise1(&numbers);
    exercise2(&numbers);
    Ok(())
}

fn exercise1(list: &Vec<i32>) {
    for index in 0..list.len() {
        let entry1 = list.get(index).unwrap();
        for index2 in index..list.len() {
            let entry2 = list.get(index2).unwrap();
            if entry1 + entry2 == 2020 {
                println!("exercise1: {}", entry1 * entry2);
                return;
            }
        }
    }
}

fn exercise2(list: &Vec<i32>) {
    for index in 0..list.len() {
        let entry1 = list.get(index).unwrap();
        for index2 in index..list.len() {
            let entry2 = list.get(index2).unwrap();
            for index3 in index..list.len() {
                let entry3 = list.get(index3).unwrap();
                if entry1 + entry2 + entry3 == 2020 {
                    println!("exercise2: {}", entry1 * entry2 * entry3);
                    return;
                }
            }
        }
    }
}