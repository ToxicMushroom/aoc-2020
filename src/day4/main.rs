use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day4/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let rows: Vec<&str> = contents.split("\r\n\r\n").collect();
    println!("total passports: {:?}", rows.len());
    exercise1(&rows);
    exercise2(&rows);
    Ok(())
}

fn exercise1(rows: &Vec<&str>) {
    let mut counter = 0;
    for index in 0..rows.len() {
        let row = rows[index];
        counter += is_valid_passport1(row) as usize;
    }
    println!("valid passports1: {:?}", counter);
}

fn is_valid_passport1(passport: &str) -> bool {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let pass_fields: Vec<&str> = passport.split(" ").flat_map(|s| s.split("\r\n")).collect();
    let mut matches: usize = 0;
    for field in pass_fields {
        let key = field.split(":").nth(0).unwrap();
        matches += required_fields.contains(&key) as usize
    }
    matches == required_fields.len()
}

fn exercise2(rows: &Vec<&str>) {
    let mut counter = 0;
    for index in 0..rows.len() {
        let row = rows[index];
        counter += is_valid_passport2(row) as usize;
    }
    println!("valid passports2: {:?}", counter);
}

fn is_valid_passport2(passport: &str) -> bool {
    let pass_fields: Vec<&str> = passport.split(" ").flat_map(|s| s.split("\r\n")).collect();
    let re = Regex::new(r"^#[0-9A-Fa-f]{6}").unwrap();
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let eye_clrs = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut matches: usize = 0;
    for field in pass_fields {
        let split: Vec<&str> = field.split(":").collect();
        let key = split[0];
        let value = split[1];
        matches += match key {
            "byr" => {
                let year: usize = value.parse().unwrap();
                year <= 2002 && year >= 1920
            }
            "iyr" => {
                let year: usize = value.parse().unwrap();
                year <= 2020 && year >= 2010
            }
            "eyr" => {
                let year: usize = value.parse().unwrap();
                year <= 2030 && year >= 2020
            }
            "hgt" => {
                if value.ends_with("cm") {
                    let cm: usize = value.trim_end_matches("cm").parse().unwrap();
                    cm <= 193 && cm >= 150
                } else {
                    let inch: usize = value.trim_end_matches("in").parse().unwrap();
                    inch <= 76 && inch >= 59
                }
            }
            "hcl" => {
                println!("{:?} - {:?}", value, re.is_match(value));
                re.is_match(value)
            }
            "ecl" => {
                eye_clrs.contains(&value)
            }
            "pid" => {
                value.len() == 9 && value.chars().all(char::is_numeric)
            }
            _ => {
                false
            }
        } as usize;
    }
    matches == required_fields.len()
}