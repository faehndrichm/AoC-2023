use num_integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

enum Direction {
    Left,
    Right,
}

fn main() {
    let start = Instant::now();

    let mut lines = match read_lines("./src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let line1 = match lines.next() {
        Some(Ok(line)) => line,
        _ => panic!("Error reading line 1"),
    };

    _ = lines.next();

    let directions: Vec<Direction> = line1
        .chars()
        .map(|d| {
            if d == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let mut map_left = HashMap::new();
    let mut map_right = HashMap::new();

    for line_res in lines {
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        let (root_label, childs) = line.split_once(" = ").expect("no = ");
        let (mut left_s, mut right_s) = childs.split_once(", ").expect("no , ");
        (_, left_s) = left_s.split_once("(").expect("a");
        (right_s, _) = right_s.split_once(")").expect("a");

        map_left.insert(root_label.to_string(), left_s.to_string());
        map_right.insert(root_label.to_string(), right_s.to_string());
    }

    let mut currents: Vec<(String, i32)> = map_left
        .keys()
        .filter_map(|k| {
            if k.ends_with("A") {
                Some((k.clone(), 0))
            } else {
                None
            }
        })
        .collect();

    for current in currents.iter_mut() {
        for dir in directions.iter().cycle() {
            if current.0.ends_with("Z") {
                break;
            }
            current.1 += 1;
            current.0 = match dir {
                Direction::Right => map_right.get(&current.0).unwrap().to_string(),
                Direction::Left => map_left.get(&current.0).unwrap().to_string(),
            };
        }
    }
    let res = currents.iter().map(|c| c.1 as u64).reduce(|a, b| lcm(a, b));
    let duration = start.elapsed();
    println!("Result: {:?}", res);
    println!("Time elapsed: {:?}", duration);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
