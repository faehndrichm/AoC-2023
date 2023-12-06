use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

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

    let line2 = match lines.next() {
        Some(Ok(line)) => line,
        _ => panic!("Error reading line 2"),
    };

    let (_, times_s) = line1.split_once(':').expect("No :");
    let times: Vec<u64> = times_s
        .split_whitespace()
        .filter_map(|f| f.parse().ok())
        .collect();
    let (_, distances_s) = line2.split_once(':').expect("No :");
    let distances: Vec<u64> = distances_s
        .split_whitespace()
        .filter_map(|f| f.parse().ok())
        .collect();

    let mut res = 1;

    println!("{:?}", times);
    println!("{:?}", distances);

    for i in 0..times.len() {
        let mut res_race = 0;

        let time = times[i];
        let distance = distances[i];
        for hold in 1..=time {
            let speed = hold;

            let distance_travelled = (time - hold) * speed;
            if distance_travelled > distance {
                res_race += 1;
            }
        }
        println!("{}", res_race);

        res = res * res_race;
    }

    let duration = start.elapsed();
    println!("Result 1: {}", res);
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
