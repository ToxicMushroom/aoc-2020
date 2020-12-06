use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day6/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let groups: Vec<&str> = contents.split("\r\n\r\n").collect();
    println!("total groups: {:?}", groups.len());
    exercise1(&groups);
    exercise2(&groups);
    Ok(())
}

fn exercise1(groups: &Vec<&str>) {
    let mut count = 0;
    for i in 0..groups.len() {
        let group = groups[i];
        let mut questions: Vec<char> = vec!();
        group.lines().for_each(|line| {
            line.chars().for_each(|c| {
                if !questions.contains(&c) {
                    questions.insert(0, c);
                }
            })
        });
        count += questions.len();
    }
    println!("questions any {:?}", count);
}

fn exercise2(groups: &Vec<&str>) {
    let mut count = 0;
    for i in 0..groups.len() {
        let group = groups[i];
        let mut questions: Vec<char> = vec!();
        let mut lines = 0;
        group.lines().for_each(|line| {
            lines += 1;
            let mut your_questions: Vec<char> = vec!();
            line.chars().for_each(|c| {
                if !your_questions.contains(&c) {
                    your_questions.insert(0, c);
                }
            });
            questions.append(your_questions.as_mut());
        });
        for j in 97..123 {
            let mut extra = 0;
            questions.iter().for_each(|a| {
                if j == *a as usize {
                    extra += 1;
                }
            });
            if extra == lines {
                count += 1;
            }
        }
    }
    println!("questions all {:?}", count);
}