use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day5/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let rows: Vec<&str> = contents.lines().collect();
    println!("total boarding: {:?}", rows.len());
    exercise1(&rows);
    exercise2(&rows);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let p1 = get_helft_helft("BFFFBBF", 'F', 'B', 0, 127);
        let p2 = get_helft_helft("RRR", 'L', 'R', 0, 7);

        assert_eq!(p1, 70);
        assert_eq!(p2, 7);
    }

    #[test]
    fn test2() {
        let p1 = get_helft_helft("FFFBBBF", 'F', 'B', 0, 127);
        let p2 = get_helft_helft("RRR", 'L', 'R', 0, 7);

        assert_eq!(p1, 14);
        assert_eq!(p2, 7);
    }

    #[test]
    fn test3() {
        let p1 = get_helft_helft("BBFFBBF", 'F', 'B', 0, 127);
        let p2 = get_helft_helft("RLL", 'L', 'R', 0, 7);

        assert_eq!(p1, 102);
        assert_eq!(p2, 4);
    }
}

fn get_helft_helft(pattern: &str, c1: char, c2: char, start: usize, end: usize) -> usize {
    let mut start_now = start;
    let mut start_end = end;
    let mut width = start_end - start_now;
    pattern.chars().for_each(|c| {
        if c == c1 {
            start_end = start_now + (width / 2)
        } else if c == c2 {
            start_now = start_end - (width / 2)
        }
        width = start_end - start_now
    });
    start_end
}

fn exercise1(rows: &Vec<&str>) {
    let mut highest_id = 0;
    for i in 0..rows.len() {
        let el = rows[i];
        let top_bottom = el.get(0..7).unwrap();
        let left_right = el.get(7..10).unwrap();
        let y_coordinate = get_helft_helft(top_bottom, 'F', 'B', 0, 127);
        let x_coordinate = get_helft_helft(left_right, 'L', 'R', 0, 7);

        let id = y_coordinate * 8 + x_coordinate;
        if id > highest_id {
            highest_id = id
        }
    }
    println!("highest: {:?}", highest_id);
}


fn exercise2(rows: &Vec<&str>) {
    let mut seat_ids: Vec<usize> = vec!();
    for i in 0..rows.len() {
        let el = rows[i];
        let top_bottom = el.get(0..7).unwrap();
        let left_right = el.get(7..10).unwrap();
        let y_coordinate = get_helft_helft(top_bottom, 'F', 'B', 0, 127);
        let x_coordinate = get_helft_helft(left_right, 'L', 'R', 0, 7);

        let id = y_coordinate * 8 + x_coordinate;
        seat_ids.insert(0, id);
    }
    seat_ids.sort();
    let start = *seat_ids.first().unwrap();
    let end = *seat_ids.last().unwrap();
    for i in start..end {
        if seat_ids.binary_search(&i).is_err() {
            // missing
            println!("empty chair id: {:?}", i);
            return;
        }
    }
}