use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut sequences = Vec::new();
    for line_res in lines {
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        let sequence: Vec<i32> = line
            .split_whitespace()
            .filter_map(|f| f.parse().ok())
            .collect();
        sequences.push(sequence);
    }
    let mut res1 = 0;
    let mut res2 = 0;

    for sequence in sequences {
        println!("Seq: {:?}", sequence);

        let mut interpolations = Vec::new();
        interpolations.push(sequence);
        let mut cont = true;
        while cont {
            let last_pol = interpolations.last().unwrap();
            let mut next_pol = Vec::with_capacity(last_pol.len() - 1);
            for window in last_pol.windows(2) {
                match window {
                    [a, b] => {
                        let diff = b - a;
                        next_pol.push(diff);
                    }
                    _ => panic!("invalid window size"),
                }
            }
            if next_pol.iter().all(|f| *f == 0) {
                cont = false;
            }
            interpolations.push(next_pol);
        }
        let mut extrapol1 = 0;
        for cur_inter in interpolations.iter() {
            extrapol1 += cur_inter.last().unwrap_or(&0);
        }
        res1 += extrapol1;

        // part
        let mut extrapol_back: i32 = 0;
        for cur_inter in interpolations.iter().rev() {
            extrapol_back = *cur_inter.first().unwrap_or(&0) - extrapol_back;
            println!("{}", extrapol_back);
        }
        res2 += extrapol_back;
    }

    let duration = start.elapsed();
    println!("Result1: {:?}", res1);
    println!("Result2: {:?}", res2);
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
