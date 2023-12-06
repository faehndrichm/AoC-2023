use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

fn main() {
    let lines = match read_lines("./src/input_4.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let base: i32 = 2;

    let mut repeats: HashMap<u32, i32> = HashMap::new();

    for (iu, line_res) in lines.enumerate() {
        let i = iu as u32;
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        let (_, numbers) = line.split_once(':').expect("No :");

        let (win_nums, my_nums) = numbers.split_once("|").expect("NO |");
        let win_set: HashSet<i32> =
            HashSet::from_iter(win_nums.split_whitespace().filter_map(|s| s.parse().ok()));
        let my_set: HashSet<i32> =
            HashSet::from_iter(my_nums.split_whitespace().filter_map(|s| s.parse().ok()));

        let win_count: u32 = win_set
            .intersection(&my_set)
            .count()
            .try_into()
            .unwrap_or(0);
        if win_count > 0 {
            sum_1 += base.pow(win_count - 1);
        }

        let repeat_count = match repeats.get(&i) {
            Some(&c) => c + 1,
            _ => 1,
        };

        for j in 1..=win_count {
            if let Some(count) = repeats.get(&(i + j)) {
                repeats.insert(i + j, count + repeat_count);
            } else {
                repeats.insert(i + j, repeat_count);
            };
        }
        sum_2 += repeat_count;
    }
    println!("Result 1: {}", sum_1);
    println!("Result 2: {}", sum_2);
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
