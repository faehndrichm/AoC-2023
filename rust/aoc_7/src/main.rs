use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, empty, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
struct Player {
    hand: [char; 5],
    //hand_value: i32,
    bid: i32,
}

fn get_letters_count(p: &Player) -> HashMap<char, i32> {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    for c in p.hand {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    return letter_counts;
}

fn compare_hands(a: &Player, b: &Player, value_map: &HashMap<char, i32>) -> Ordering {
    let letter_counts_a: HashMap<char, i32> = get_letters_count(&a);
    let letter_counts_b: HashMap<char, i32> = get_letters_count(&b);

    let j_count_a = *letter_counts_a.get(&'J').unwrap_or(&0);
    let j_count_b = *letter_counts_b.get(&'J').unwrap_or(&0);

    println!("{:?}", a.hand);
    println!("{:?}", b.hand);

    println!("{}", j_count_a);
    println!("{}", j_count_b);

    let mut counts_a: Vec<i32> = letter_counts_a.into_values().collect();
    let mut counts_b: Vec<i32> = letter_counts_b.into_values().collect();
    counts_a.sort_by(|a, b| b.cmp(a));
    counts_b.sort_by(|a, b| b.cmp(a));

    if j_count_a > 0 && j_count_a != 5 {
        let index_j = counts_a.iter().position(|x| *x == j_count_a).unwrap();
        counts_a.remove(index_j);
        counts_a[0] += j_count_a;
    }

    if j_count_b > 0 && j_count_b != 5 {
        let index_j = counts_b.iter().position(|x| *x == j_count_b).unwrap();
        counts_b.remove(index_j);
        counts_b[0] += j_count_b;
    }

    println!("{:?}", counts_a);
    println!("{:?}", counts_b);

    if counts_a == counts_b {
        //2. rule compare letters
        println!("compare second function");
        for i in 0..5 {
            let val_a = value_map.get(&a.hand[i]).unwrap_or(&0);
            let val_b = value_map.get(&b.hand[i]).unwrap_or(&0);

            if val_a > val_b {
                println!("Greater");
                return Ordering::Greater;
            } else if val_a < val_b {
                println!("Less");
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }

    if counts_a == [5] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [5] {
        return Ordering::Less;
    }

    if counts_a == [4, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [4, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [3, 2] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [3, 2] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [3, 1, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [3, 1, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [3, 1, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [3, 1, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [2, 2, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [2, 2, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [2, 1, 1, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [2, 1, 1, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [2, 2, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [2, 2, 1] {
        println!("Less");
        return Ordering::Less;
    }

    if counts_a == [2, 1, 1, 1] {
        println!("Greater");
        return Ordering::Greater;
    } else if counts_b == [2, 1, 1, 1] {
        println!("Less");
        return Ordering::Less;
    }

    panic!("I think we should not reach this");

    return Ordering::Equal;
}

// fn get_hand_value(letters: Vec<char>, value_map: &HashMap<char, i32>) -> i32 {
//     let mut letter_counts: HashMap<char, i32> = HashMap::new();
//     let mut sum = 0;
//     for c in letters {
//         *letter_counts.entry(c).or_insert(0) += 1;
//     }

//     let base: i32 = 10;
//     for (letter, &count) in letter_counts.iter() {
//         sum += base.pow(count as u32) * value_map.get(letter).unwrap();
//     }

//     return sum;
// }

fn main() {
    let start = Instant::now();
    let mut res = 0;

    let mut players: Vec<Player> = Vec::new();
    let value_map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
    let lines = match read_lines("./src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    for (iu, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        let (hand_s, bid_s) = line.split_once(' ').expect("No :");

        let letters: Vec<char> = hand_s.chars().collect();
        players.push(Player {
            hand: letters.try_into().unwrap(),
            bid: bid_s.parse().unwrap_or(0),
        })
    }

    let duration = start.elapsed();
    println!("players: {:?}", players);
    players.sort_by(|a, b| compare_hands(a, b, &value_map));
    println!("sorted: {:?}", players);
    res = players
        .iter()
        .enumerate()
        .map(|(i, p)| (i as i32 + 1) * p.bid)
        .sum();
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
