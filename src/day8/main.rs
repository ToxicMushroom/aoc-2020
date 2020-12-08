use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day8/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let instructions: Vec<&str> = contents.lines().collect();
    exercise(&instructions);
    exercise2(&instructions);
    Ok(())
}

fn exercise2(instructions: &Vec<&str>) {
    let mut count = 0;
    loop {
        count += 1;
        let mut r_type = false;

        if count == instructions.len() && !r_type {
            count = 0;
            r_type = true;
        } else if count == instructions.len() {
            break;
        }

        let mut accum = 0;
        let mut bevelen_teller: isize = 0;
        let mut positions: Vec<isize> = vec! {};
        loop {
            if bevelen_teller == instructions.len() as isize {
                break;
            }
            let parts: Vec<&str> = instructions[bevelen_teller as usize].split(" ").collect();
            let mut op = parts[0];
            if bevelen_teller == count as isize {
                if !r_type && op == "jmp" {
                    op = "nop"
                } else if r_type && op == "nop" {
                    op = "jmp"
                };
            }

            match op {
                "nop" => {
                    bevelen_teller += 1;
                }
                "acc" => {
                    let value: isize = parts[1].parse().unwrap();
                    accum += value;

                    bevelen_teller += 1;
                }
                "jmp" => {
                    let value: isize = parts[1].parse().unwrap();

                    if positions.contains(&bevelen_teller) {
                        break;
                    } else {
                        positions.insert(0, bevelen_teller);
                    }
                    bevelen_teller += value;
                }
                _ => {
                    continue;
                }
            }
        }
        if bevelen_teller == instructions.len() as isize {
            println!("{:?} - {:?}", accum, count);
            break;
        }
    }
}

fn exercise(instructions: &Vec<&str>) {
    let mut accum = 0;
    let mut bevelen_teller: isize = 0;
    let mut positions: Vec<isize> = vec! {};
    let mut prev_loc = 0;
    loop {
        let parts: Vec<&str> = instructions[bevelen_teller as usize].split(" ").collect();
        let op = parts[0];
        match op {
            "nop" => {
                bevelen_teller += 1;
            }
            "acc" => {
                let value: isize = parts[1].parse().unwrap();
                accum += value;

                bevelen_teller += 1;
            }
            "jmp" => {
                let value: isize = parts[1].parse().unwrap();

                if positions.contains(&bevelen_teller) {
                    break;
                } else {
                    prev_loc = accum;
                    positions.insert(0, bevelen_teller);
                }
                bevelen_teller += value;
            }
            _ => {
                continue;
            }
        }
    }
    println!("{:?}", prev_loc);
}