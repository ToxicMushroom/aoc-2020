use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::borrow::Borrow;

fn main() -> std::io::Result<()> {
    let file = File::open("./src/day7/input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let rules: Vec<&str> = contents.lines().collect();
    exercise(&rules);
    Ok(())
}

fn exercise(groups: &Vec<&str>) {
    let mut bag_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    // create bag_map
    for i in 0..groups.len() {
        let rule = groups[i];
        let bags: Vec<&str> = rule.split(" contain ").collect();
        let bag1 = bags[0];
        let dest2: Vec<&str> = bags[1].get(0..(bags[1].len() - 1)).unwrap().split(", ").collect();
        let mut dests_out: Vec<(String, usize)> = vec!();

        dest2.iter().for_each(|bag2| {
            if *bag2 != "no other bags" {
                let split_location = bag2.find(" ").unwrap();
                let bag_count: usize = bag2.get(0..split_location).unwrap().parse().expect("parse error");
                let bag_type = bag2.get((split_location + 1)..bag2.len()).unwrap();

                if bag_count == 1 {
                    let mut str = String::from(bag_type);
                    str.push('s');

                    dests_out.insert(0, (str, bag_count));
                } else {
                    dests_out.insert(0, (bag_type.to_owned(), bag_count));
                }
            }
        });
        bag_map.insert(bag1.to_string(), dests_out);
    }

    // excecise1
    let mut count = 0;
    for x in bag_map.borrow() {
        if found_deep_bag(&bag_map, &x.0.to_string()) {
            count += 1;
        }
    }
    println!("{:?}", count);

    // excecise2
    let count2 = count_deep_bags(&bag_map, &String::from("shiny gold bags"), true);
    println!("{:?}", count2);
}


fn count_deep_bags(bag_map: &HashMap<String, Vec<(String, usize)>>, bag: &String, root: bool) -> usize {
    let mut count = 0;
    let value_opt: &Option<&Vec<(String, usize)>> = &bag_map.get(bag);

    let value: &Vec<(String, usize)> = value_opt.unwrap();
    if value.len() == 0 {
        return 1;
    }

    let parent_count = if root { 0 } else { 1 };
    for one in value {
        let deep = count_deep_bags(bag_map, &one.0, false);
        count += one.1 * deep;
    }
    parent_count + count
}

fn found_deep_bag(bag_map: &HashMap<String, Vec<(String, usize)>>, bag: &String) -> bool {
    let value_opt: &Option<&Vec<(String, usize)>> = &bag_map.get(bag);

    let value: &Vec<(String, usize)> = value_opt.unwrap();
    if value.len() == 0 {
        return false;
    }

    for one in value {
        if one.0 == "shiny gold bags" {
            return true;
        }
    }

    for one in value {
        if found_deep_bag(bag_map, &one.0) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut bag_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();
        bag_map.insert(String::from("bag1"), vec! {(String::from("a"), 2)});
        bag_map.insert(String::from("a"), vec! {});

        let p1 = count_deep_bags(&bag_map, &String::from("bag1"), true);

        bag_map.insert(String::from("a"), vec! {(String::from("b"), 3), (String::from("c"), 3)});
        bag_map.insert(String::from("b"), vec! {});
        bag_map.insert(String::from("c"), vec! {(String::from("b"), 3)});

        let p2 = count_deep_bags(&bag_map, &String::from("bag1"), true);

        assert_eq!(p1, 2);
        assert_eq!(p2, 32);
    }
}

// fn exercise2(groups: &Vec<&str>) {
//
// }