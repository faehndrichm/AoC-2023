use std::fs::File;
use std::io::{self, empty, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
struct SeedMapping {
    start: u64,
    end: u64,
    incr: i64,
}

#[derive(Clone, Debug)]
struct SeedRange {
    //id: usize,
    start: u64,
    end: u64,
    //typ: String,
}

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut cat_id: i64 = -1;
    //let mut seeds: Vec<u64> = Vec::new();
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let mut categories: Vec<Vec<SeedMapping>> = Vec::new();

    for (iu, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        if line.contains(":") && cat_id == -1 {
            cat_id += 1;
            let (_, num_s) = line.split_once(':').expect("No :");
            let seeds_ranges_s: Vec<u64> = num_s
                .split_whitespace()
                .filter_map(|f| f.parse().ok())
                .collect();
            // for (i, &seed) in seeds_ranges.iter().enumerate() {
            //     if (i % 2 == 1) {
            //         let last_seed = seeds_ranges[i - 1];
            //         seeds.extend(last_seed..seed + last_seed);
            //     }
            // }
            for (i, &seed) in seeds_ranges_s.iter().enumerate() {
                if i % 2 == 1 {
                    let last_seed = seeds_ranges_s[i - 1];
                    seed_ranges.push(SeedRange {
                        //id: i,
                        start: (last_seed),
                        end: (seed + last_seed),
                        //typ: "init".to_string(),
                    });
                }
            }
        } else if line.contains(":") {
            categories.push(Vec::new());
            cat_id = (categories.len() - 1) as i64;
        } else {
            //categories[i]
            let ranges: Vec<u64> = line
                .split_whitespace()
                .filter_map(|f| f.parse().ok())
                .collect();
            if ranges.len() != 3 {
                continue;
            }
            categories[cat_id as usize].push(SeedMapping {
                start: ranges[1],
                end: ranges[1] + ranges[2],
                incr: (ranges[0] as i64 - ranges[1] as i64),
            });
        }
    }

    // println!("Seeds: {:?}", seeds);

    let mut values: Vec<SeedRange> = seed_ranges;

    for cat_maps in categories.iter() {
        let mut next_category_values: Vec<SeedRange> = Vec::new();
        let mut next_mapping_values: Vec<SeedRange>;
        for cat_map in cat_maps {
            next_mapping_values = Vec::new();
            for val in values {
                if val.end < cat_map.start || val.start > cat_map.end {
                    // outside no mapping
                    next_mapping_values.push(val.clone());
                    continue;
                }
                if val.start < cat_map.start {
                    // smaller than mapping create new range
                    next_mapping_values.push(SeedRange {
                        // id: 0,
                        start: val.start,
                        end: val.end.min(cat_map.start - 1),
                        //typ: "start".to_string(),
                    });
                }
                if val.end > cat_map.end {
                    // bigger than mapping create new range
                    next_mapping_values.push(SeedRange {
                        //id: 0,
                        start: val.start.max(cat_map.end - 1),
                        end: val.end,
                        //typ: "end".to_string(),
                    });
                }
                if val.start <= cat_map.end || val.end >= cat_map.start {
                    //overlap
                    next_category_values.push(SeedRange {
                        //id: 0,
                        start: (val.start.max(cat_map.start) as i64 + cat_map.incr) as u64,
                        end: (val.end.min(cat_map.end) as i64 + cat_map.incr) as u64,
                        //typ: "modified".to_string(),
                    });
                }
            }
            values = next_mapping_values;
        }
        values.append(&mut next_category_values);
        //println!("values: {:?}", values);
    }

    let min = values.iter().map(|r| r.start).min().unwrap();
    let duration = start.elapsed();

    println!("Result 1: {}", min);
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
