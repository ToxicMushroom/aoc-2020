use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day2/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    exercise1(&contents);
    exercise2(&contents);
    Ok(())
}

fn exercise1(content: &str) {
    let list: Vec<&str> = content.split("\n").collect();
    let mut counter = 0;
    for index in 0..list.len() {
        let entry1 = list[index];
        let parts: Vec<&str> = entry1.split_whitespace().collect();
        let range = parts[0];
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2];
        let range_parts: Vec<&str> = range.split("-").collect();
        let start: i32 = range_parts[0].parse().expect("parse error");
        let end: i32 = range_parts[1].parse().expect("parse error");

        let mut count = 0;
        for c in password.chars() {
            if c == letter {
                count += 1;
            }
        }
        if count >= start && count <= end {
            counter += 1;
        }
    }
    println!("{:?}", counter);
}

fn exercise2(content: &str) {
    let list: Vec<&str> = content.split("\n").collect();
    let mut counter = 0;
    for index in 0..list.len() {
        let entry1 = list[index];
        let parts: Vec<&str> = entry1.split_whitespace().collect();
        let range = parts[0];
        let letter = parts[1].as_bytes()[0];
        let password = parts[2].as_bytes();
        let range_parts: Vec<&str> = range.split("-").collect();
        let one: usize = range_parts[0].parse().expect("parse error");
        let two: usize = range_parts[1].parse().expect("parse error");

        if (password[one-1] == letter) ^ (password[two-1] == letter) {
            counter += 1;
        }
    }
    println!("{:?}", counter);
}
